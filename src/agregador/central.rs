use std::process::id;
use std::sync::Arc;
use crate::entidades::modelos::{Alerta, EstacaoCosteira, EstacaoMontanha, EstacaoSuperficie, Leitura, Severidade, TipoAlerta, TipoEstacao};
use crate::erros::erros_suportados::Erros;
use crate::relatorios::modelo_relatorio::{FormatoRelatorio, RelatorioResumido, TipoRelatorio};

pub struct estacao_central{
    pub estacoes: Vec<Arc<dyn EstacaoMeteorologica>>,
    pub leituras: Vec<Leitura>,
    pub alertas: Vec<Alerta>,
    pub ciclo_atual: u32 //numero do ciclo de coleta
}

pub trait EstacaoMeteorologica : Send + Sync {

    fn id(&self) -> u32;
    fn nome_local(&self) -> &str;
    fn tipo(&self) -> TipoEstacao;
    fn coordenadas(&self) -> (f64, f64);
    fn temperatura(&self) -> f64;
    fn vento(&self) -> f64;
    fn e_critico(&self) -> bool; //cada tipo checa seu próprio limite (temp, vento ou geada)
    fn tipo_alerta(&self) -> TipoAlerta; //TipoAlerta — qual alerta este tipo emite quando crítico
    fn descricao_alerta(&self) -> String;
    fn nivel_severidade(&self) -> Severidade;
    fn ficha(&self) -> String{
        format!("{} tipo @ {} - {}°C / {}km/h", self.tipo().representar_tipo_estacao(), 
                self.nome_local(), self.temperatura(), self.vento())
    }

}

//seção de operações
impl estacao_central{

    pub fn registrar_leitura(&mut self, estacao_id: u32, ciclo: u32) -> Result<Leitura, Erros>{

        let selecionada_estacao:Option<&dyn EstacaoMeteorologica> = self.estacoes.iter()
            .find(|f| f.id() == estacao_id)
            .map(|m| m.as_ref());
        if selecionada_estacao.is_none(){
            return Err(Erros::EstacaoNaoEncontrada(estacao_id));
        }
        let estacao = selecionada_estacao.unwrap();
        let leitura = Leitura::novo(estacao.id(), estacao.tipo(), estacao.temperatura(), estacao.vento(), ciclo, estacao.e_critico());
        self.leituras.push(leitura.clone());
        if estacao.e_critico(){
            self.alertas.push(Alerta::novo(estacao.id(),
                                           estacao.nome_local().to_string(), estacao.tipo_alerta(),
                                           estacao.nivel_severidade(), estacao.temperatura(),
                                           estacao.descricao_alerta(),
                                           ciclo))
        }
        Ok(leitura)
    }

    pub fn adicionar_estacao(&mut self, estacao: Arc<dyn EstacaoMeteorologica>){
        self.estacoes.push(estacao);
    }

    pub fn varrer_rede(&mut self) -> Vec<Leitura>{

        let estacoes:Vec<&Arc<dyn EstacaoMeteorologica>> = self.estacoes.iter().collect();
        let mut id_ciclo:Vec<(u32, u32)> = Vec::new();
        let mut leituras_registradas: Vec<Leitura> = Vec::new();

        for (idx, i) in estacoes.iter().enumerate(){
            id_ciclo.push((i.id(), idx as u32));
        }

        for i in id_ciclo{
           if let Ok(x) = self.registrar_leitura(i.0, i.1){
               leituras_registradas.push(x);
           }
        }
        leituras_registradas
    }

    pub fn estacoes_criticas(&self) -> Vec<Arc<dyn EstacaoMeteorologica>>{
        self.estacoes.iter().filter(|f| f.e_critico()).cloned().collect()
    }

}

//seção de consultas
impl estacao_central{
    pub fn leituras_por_estacao(&self, estacao_id: u32) -> Vec<&Leitura>{
        self.leituras.iter().filter(|f| f.estacao_id == estacao_id).collect()
    }

    pub fn estacao_mais_quente(&self) -> Option<&Leitura> {

        //quando a comparação envolver f64 por conta das restrições de ord envolvendo NaN
        //utilizar total_cmp do tipo f64::total_cmp sendo possivel realizar esta ordenação
        //pois um tipo e apenas suportado por ord quando (a > b ou a < b ou a == b)
        //e NaN não satisfaz nenhuma condição
        self.leituras.iter().max_by(|a,b| a.temperatura.total_cmp(&b.temperatura))
    }

    pub fn gerar_relatorio(&self, tipo_relatorio: TipoRelatorio, ciclo: u32) -> Box<dyn FormatoRelatorio>{

        let numero_leituras = self.leituras.iter().filter(|f| f.ciclo == ciclo).count();
        let numero_alertas = self.alertas.iter().filter(|f| f.ciclo == ciclo).count();

        let mut alerta_tipo:Vec<(TipoAlerta, usize)> = Vec::new();
        let mut calor: usize = 1;
        let mut vento: usize = 1;
        let mut frio:  usize = 1;

        for i in &self.alertas{
            match i.tipo_alerta {
                TipoAlerta::CalorExtremo => {
                    alerta_tipo.push((i.tipo_alerta.clone(), calor));
                    calor+=1;
                }
                TipoAlerta::VentoForte => {
                    alerta_tipo.push((i.tipo_alerta.clone(), vento));
                    vento+=1;
                }
                TipoAlerta::Geada => {
                    alerta_tipo.push((i.tipo_alerta.clone(), frio));
                    frio+=1;
                }
                _ => ()
            }
        }

        Box::new(RelatorioResumido::new(ciclo, numero_alertas, numero_leituras, alerta_tipo))

    }

}

impl EstacaoMeteorologica for EstacaoSuperficie{
    fn id(&self) -> u32 {
        self.id
    }

    fn nome_local(&self) -> &str {
        self.nome_local.as_str()
    }

    fn tipo(&self) -> TipoEstacao {
        TipoEstacao::Superficie
    }

    fn coordenadas(&self) -> (f64, f64) {
        (self.latitude, self.longitude)
    }

    fn temperatura(&self) -> f64 {
        self.temperatura_atual
    }

    fn vento(&self) -> f64 {
        self.vento_kmh
    }

    fn e_critico(&self) -> bool {
        if self.limite_temperatura_critica > 35.0{
            return true
        }
        false
    }

    fn tipo_alerta(&self) -> TipoAlerta {
        if self.e_critico(){
            return TipoAlerta::CalorExtremo
        }
        TipoAlerta::SemOcorrencia
    }

    fn descricao_alerta(&self) -> String {
        format!("alerta gerado: {}", self.tipo_alerta())
    }

    fn nivel_severidade(&self) -> Severidade {
        match self.limite_temperatura_critica {
            35.0..=40.0 => {
                Severidade::Baixa
            },
            41.0..=50.0 => {
                Severidade::Moderada
            }
            51.0..=60.0 => {
                Severidade::Alta
            }
            _ => Severidade::Baixa
        }
    }
}

impl EstacaoMeteorologica for EstacaoCosteira{
    fn id(&self) -> u32 {
        self.id
    }

    fn nome_local(&self) -> &str {
        self.nome_local.as_str()
    }

    fn tipo(&self) -> TipoEstacao {
        TipoEstacao::Costeira
    }

    fn coordenadas(&self) -> (f64, f64) {
        (self.latitude, self.longitude)
    }

    fn temperatura(&self) -> f64 {
        self.temperatura_atual
    }

    fn vento(&self) -> f64 {
        self.vento_kmh
    }

    fn e_critico(&self) -> bool {
        if self.limite_vento_critico > 60.0 {
            return true
        }
        false
    }

    fn tipo_alerta(&self) -> TipoAlerta {
        if self.e_critico(){
            return TipoAlerta::VentoForte
        }
        TipoAlerta::SemOcorrencia
    }

    fn descricao_alerta(&self) -> String {
        format!("alerta gerado: {}", self.tipo_alerta())
    }

    fn nivel_severidade(&self) -> Severidade {
        match self.limite_vento_critico {
            60.0..=70.0 => {
                Severidade::Baixa
            }
            71.0..=100.0 => {
                Severidade::Moderada
            }
            101.0..=150.0 => {
                Severidade::Alta
            }
            _ => Severidade::Baixa
        }
    }
}

impl EstacaoMeteorologica for EstacaoMontanha{
    fn id(&self) -> u32 {
        self.id
    }

    fn nome_local(&self) -> &str {
        self.nome_local.as_str()
    }

    fn tipo(&self) -> TipoEstacao {
        TipoEstacao::Montanha
    }

    fn coordenadas(&self) -> (f64, f64) {
        (self.latitude, self.longitude)
    }

    fn temperatura(&self) -> f64 {
        self.temperatura_atual
    }

    fn vento(&self) -> f64 {
        self.vento_kmh
    }

    fn e_critico(&self) -> bool {
        if self.limite_temperatura_congelamento < 0.00 {
            return true
        }
        false
    }

    fn tipo_alerta(&self) -> TipoAlerta {
        if self.e_critico(){
            return TipoAlerta::Geada
        }
        return TipoAlerta::SemOcorrencia
    }

    fn descricao_alerta(&self) -> String {
        format!("alerta gerado: {}", self.tipo_alerta())
    }

    fn nivel_severidade(&self) -> Severidade {
        match self.limite_temperatura_congelamento {
            0.00..=0.10 => {
                Severidade::Baixa
            }
            0.11..=0.20 => {
                Severidade::Moderada
            }
            0.21..=0.50 => {
                Severidade::Alta
            }
            _ => Severidade::Baixa
        }
    }
}