use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::agregador::central::EstacaoMeteorologica;
use crate::erros::erros_suportados::Erros;
use crate::entidades::modelos::{EstacaoSuperficie, TipoEstacao};

#[derive(Debug, Serialize, Deserialize)]
pub struct RespostaOpenMeteo{
    pub current_weather: Clima
}

impl RespostaOpenMeteo{
    pub fn novo() -> Self {
        Self{
            current_weather: Clima{temperature: 0.00, windspeed: 0.00}
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Clima{
    pub temperature: f64,
    pub windspeed: f64
}

impl Clima{
    fn novo() -> Self {
        Self{
            temperature: 0.00, windspeed: 0.00
        }
    }
}

pub trait suporte_comm_api{
    fn buscar_clima(&self, lat: f64, lon: f64) -> Result<RespostaOpenMeteo, Erros>{

        let mut json_resposta:RespostaOpenMeteo = RespostaOpenMeteo::novo();
        let client = reqwest::blocking::Client::new();
        let resposta = client.get(format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true", lat, lon)).send();
        if let Ok(resp) = resposta{
            if resp.status().is_success(){

                //forma de gerar o debug ao consumir a api e reqwest nao produzir
                // uma resposta suficientemente descritiva

                /*let conteudo = resp.text().unwrap();
                let res:Result<RespostaOpenMeteo, serde_json::Error> = serde_json::from_str(&conteudo);
                match res {
                    Ok(x) => println!("{:?}", x),
                    Err(x) => println!("{:?}", x)
                }*/

                let serializacao = resp.json::<RespostaOpenMeteo>();
                if serializacao.is_err() {
                    return serializacao.map_err(|e| Erros::FalhaSerializacao(e.to_string()))
                }
                json_resposta = serializacao.unwrap();
            }
        }
        Ok(json_resposta)
    }

    fn atualizar_estacoes_da_api(&mut self, estacoes: Vec<&Arc<dyn EstacaoMeteorologica>>) -> Result<usize, Erros> {

        let mut estacoes_retorno:Vec<Arc<dyn EstacaoMeteorologica>> = Vec::with_capacity(estacoes.len());

        for obj in estacoes{
            if let Ok(x) = self.buscar_clima(obj.coordenadas().0, obj.coordenadas().1){
                match obj.tipo() {
                    TipoEstacao::Superficie => {
                        let estacao_superficie = EstacaoSuperficie::novo(obj.id(), obj.nome_local().to_string(),
                                                                         obj.coordenadas().0, obj.coordenadas().1, x.current_weather.temperature,
                                                                         x.current_weather.windspeed, 35.00);
                        estacoes_retorno.push(Arc::new(estacao_superficie));
                    }
                    //TODO CONCLUIR AS DEMAIS ESTACOES
                    _ => ()
                };
            }
        }
        Ok(estacoes_retorno.len())

    }

}

impl suporte_comm_api for RespostaOpenMeteo {}