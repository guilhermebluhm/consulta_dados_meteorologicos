use crate::entidades::modelos::TipoAlerta;

#[derive(Debug)]
pub struct RelatorioResumido{
    pub ciclo: u32, //O ciclo de coleta reportado.
    pub total_leituras: usize, //Quantas leituras aconteceram neste ciclo.
    pub total_alertas: usize, //Quantos alertas foram disparados neste ciclo.
    pub alertas_por_tipo: Vec<(TipoAlerta, usize)> //Contagem agregada — quantos de cada tipo (CalorExtremo, VentoForte, Geada).
}

#[derive(Debug)]
pub enum TipoRelatorio{
    Resumido
}

pub trait FormatoRelatorio{
    fn titulo(&self) -> &str;
    fn formatar(&self) -> String;
    fn total_registros(&self) -> usize;
}

