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

impl EstacaoSuperficie{
    pub fn novo(id: u32, nome_local: String, latitude: f64, longitude: f64, temperatura_atual: f64, vento_kmh: f64, limite_temperatura_critica: f64) -> Self{
        Self{
            id, nome_local, latitude, longitude, temperatura_atual, vento_kmh, limite_temperatura_critica
        }
    }
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

impl EstacaoCosteira{
    pub fn novo(id: u32, nome_local: String, latitude: f64, longitude: f64, temperatura_atual: f64, vento_kmh: f64, limite_vento_critico: f64) -> Self{
        Self{
            id, nome_local, latitude, longitude, temperatura_atual, vento_kmh, limite_vento_critico
        }
    }
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

impl EstacaoMontanha{
    pub fn novo(id: u32, nome_local: String, latitude: f64, longitude: f64, altitude_m: f64, temperatura_atual: f64, vento_kmh: f64, limite_temperatura_congelamento:f64) -> Self{
        Self{
            id, nome_local, latitude, longitude, altitude_m, temperatura_atual, vento_kmh, limite_temperatura_congelamento
        }
    }
}

#[derive(Debug, Clone)]
pub enum TipoEstacao{
    Superficie,
    Costeira,
    Montanha
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Leitura{
    pub estacao_id: u32,
    pub tipo_estacao: TipoEstacao,
    pub temperatura: f64,
    pub vento_kmh: f64,
    pub ciclo: u32, //numero do ciclo de coleta
    pub critico: bool
}

impl Leitura{
    pub fn novo(estacao_id: u32, tipo_estacao: TipoEstacao, temperatura: f64,
                vento_kmh: f64, ciclo: u32, critico: bool) -> Self{
        Self{
            estacao_id, tipo_estacao, temperatura, vento_kmh, ciclo, critico
        }
    }
}

#[derive(Debug)]
pub struct Alerta{
    pub estacao_id: u32,
    pub nome_local: String,
    pub tipo_alerta: TipoAlerta,
    pub severidade: Severidade,
    pub valor_registrado: f64,
    pub mensagem: String,
    pub ciclo: u32
}

impl Alerta{
    pub fn novo(estacao_id: u32, nome_local: String, tipo_alerta: TipoAlerta,
                severidade: Severidade, valor_registrado: f64, mensagem: String, ciclo: u32) -> Self{
        Self{
            estacao_id, nome_local, tipo_alerta, severidade, valor_registrado, mensagem, ciclo
        }
    }
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