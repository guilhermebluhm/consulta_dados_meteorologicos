use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct EstacaoSuperficie{
    pub id: u32,
    pub nome_local: String,
    pub latitude: f64,
    pub longitude: f64,
    pub temperatura_atual: f64,
    pub vento_kmh: f64,
    pub limite_temperatura_critica: f64,
}

#[derive(Debug)]
pub struct EstacaoCosteira{
    pub id: u32,
    pub nome_local: String,
    pub latitude: f64,
    pub longitude: f64,
    pub temperatura_atual: f64,
    pub vento_kmh: f64,
    pub limite_vento_critico: f64,
}

#[derive(Debug)]
pub struct EstacaoMontanha{
    pub id: u32,
    pub nome_local: String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude_m: f64,
    pub temperatura_atual: f64,
    pub vento_kmh: f64,
    pub limite_temperatura_congelamento: f64,
}

#[derive(Debug, Clone)]
pub enum TipoEstacao{
    Superficie,
    Costeira,
    Montanha
}

#[derive(Debug)]
pub enum TipoAlerta{
    CalorExtremo,
    VentoForte,
    Geada,
    SemOcorrencia
}

#[derive(Debug)]
pub enum Severidade{
    Alta,
    Moderada,
    Baixa
}

pub struct Leitura{
    pub estacao_id: u32,
    pub tipo_estacao: TipoEstacao,
    pub temperatura: f64,
    pub vento_kmh: f64,
    pub ciclo: u32, //numero do ciclo de coleta
    pub critico: bool
}

pub struct Alerta{
    pub estacao_id: u32,
    pub nome_local: String,
    pub tipo_alerta: TipoAlerta,
    pub severidade: Severidade,
    pub valor_registrado: f64,
    pub mensagem: String,
    pub ciclo: u32
}

impl fmt::Display for TipoAlerta{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let texto_formatado = match self {
            TipoAlerta::CalorExtremo => "Calor",
            TipoAlerta::VentoForte => "Ventania",
            TipoAlerta::Geada => "Frio",
            TipoAlerta::SemOcorrencia => "Sem ocorrencia",
        };
        write!(f, "{}", texto_formatado)
    }
}

//implementacao do display de TipoAlerta
impl TipoEstacao{
    pub fn representar_tipo_estacao(&self) -> &str {
        match self { 
            TipoEstacao::Superficie => "Superficie",
            TipoEstacao::Costeira => "Costeira",
            TipoEstacao::Montanha => "Montanha"
        }
    }
}