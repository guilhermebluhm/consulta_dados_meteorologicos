use crate::entidades::modelos::TipoAlerta;

#[derive(Debug)]
pub struct RelatorioResumido{
    pub ciclo: u32, //O ciclo de coleta reportado.
    pub total_leituras: usize, //Quantas leituras aconteceram neste ciclo.
    pub total_alertas: usize, //Quantos alertas foram disparados neste ciclo.
    pub alertas_por_tipo: Vec<(TipoAlerta, usize)> //Contagem agregada — quantos de cada tipo (CalorExtremo, VentoForte, Geada).
}

impl RelatorioResumido{
    pub fn new(ciclo: u32, total_leituras: usize, total_alertas: usize, alertas_por_tipo: Vec<(TipoAlerta, usize)>) -> Self{
        Self{
            ciclo,
            total_alertas,
            total_leituras,
            alertas_por_tipo
        }
    }
}

#[derive(Debug)]
pub enum TipoRelatorio{
    Resumido
}

pub trait FormatoRelatorio{
    fn titulo_conteudo(&self) -> Box<str>;
    fn total_registros(&self) -> usize;
}

impl FormatoRelatorio for RelatorioResumido{
    fn titulo_conteudo(&self) -> Box<str> {
        let mut titulo_conteudo = String::from(format!("ciclo: {} - leituras: {} - alertas: {}", self.ciclo,
                                                       self.total_leituras, self.total_alertas));
        titulo_conteudo.push_str("\n");
        for (idx, ele) in self.alertas_por_tipo.iter(){
            titulo_conteudo.push_str(format!("tipo alerta {} - ocorrencias: {}", idx, ele).as_str());
        }
        //o box nao atua como uma camada de indireção neste caso
        //está efetivamente consumindo os bytes e tomando sua posse
        titulo_conteudo.into_boxed_str()
    }

    fn total_registros(&self) -> usize {
        1 + self.titulo_conteudo().len()
    }
}

