pub struct EstacaoSuperficie{
    pub id: u32,
    pub nome_local: String,
    pub latitude: f64,
    pub longitude: f64,
    pub temperatura_atual: f64,
    pub vento_kmh: f64,
    pub limite_temperatura_critica: f64
}

pub struct EstacaoCosteira{
    pub id: u32,
    pub nome_local: String,
    pub latitude: f64,
    pub longitude: f64,
    pub temperatura_atual: f64,
    pub vento_kmh: f64,
    pub limite_vento_critico: f64
}

pub struct EstacaoMontanha{
    pub id: u32,
    pub nome_local: String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude_m: f64,
    pub temperatura_atual: f64,
    pub vento_kmh: f64,
    pub limite_temperatura_congelamento: f64
}

pub enum TipoEstacao{
    Superficie,
    Costeira,
    Montanha
}

pub enum TipoAlerta{
    CalorExtremo,
    VentoForte,
    Geada
}

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

//implementacao do display de TipoAlerta
impl TipoAlerta{
    pub fn representar_tipo_alerta(&self) -> &str {
        match self { 
            TipoAlerta::CalorExtremo => "Fogo",
            TipoAlerta::VentoForte => "Ventania",
            TipoAlerta::Geada => "Frio",
        }
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