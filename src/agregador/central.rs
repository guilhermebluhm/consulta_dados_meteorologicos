use std::sync::Arc;
use crate::entidades::modelos::{Alerta, EstacaoCosteira, EstacaoMontanha, EstacaoSuperficie, Leitura, TipoAlerta, TipoEstacao};

pub struct estacao_central{
    pub estacoes: Vec<Arc<dyn EstacaoMeteorologica>>,
    leituras: Vec<Leitura>,
    alertas: Vec<Alerta>,
    ciclo_atual: u32 //numero do ciclo de coleta
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
    fn ficha(&self) -> String{
        format!("{} tipo @ {} - {}°C / {}km/h", self.tipo().representar_tipo_estacao(), 
                self.nome_local(), self.temperatura(), self.vento())
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
}