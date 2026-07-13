use std::sync::Arc;
use crate::entidades::modelos::{Alerta, Leitura, TipoAlerta, TipoEstacao};

pub struct estacao_central{
    pub estacoes: Vec<Arc<dyn EstacaoMeteorologica>>,
    leituras: Vec<Leitura>,
    alertas: Vec<Alerta>,
    ciclo_atual: u32 //numero do ciclo de coleta
}

pub trait EstacaoMeteorologica : Send + Sync {

    fn id(&self) -> String;
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