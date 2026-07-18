#[derive(Debug)]
pub enum Erros{
    EstacaoNaoEncontrada(u32), //Busca por id sem sucesso
    RedeVazia, //Operação que exige ao menos uma estação
    ValorInvalido(String), //Medição fisicamente impossível
    FalhaSerializacao(String), //serde falhou
    FalhaIO(String), //Leitura/escrita em disco
    FalhaRequisicao(String), //Falha ao chamar a API
    RespostaInvalida(String), //JSON da API não bateu com o formato.
}