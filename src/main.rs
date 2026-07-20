mod entidades;
mod agregador;
mod relatorios;
mod erros;
mod api;

use std::collections::HashMap;
use std::sync::Arc;
use entidades::modelos::{EstacaoSuperficie, EstacaoCosteira, EstacaoMontanha};
use crate::agregador::central::EstacaoMeteorologica;
use crate::api::openmeteo::{suporte_comm_api, RespostaOpenMeteo};

fn main() {

    /* (testes ja validados)
    let mut central = estacao_central {
        estacoes: Vec::new(),
        leituras: Vec::new(),
        alertas: Vec::new(),
        ciclo_atual: 1,
    };

    let superficie = EstacaoSuperficie {
        id: 1,
        nome_local: "Curitiba - Centro".to_string(),
        latitude: -25.42,
        longitude: -49.27,
        temperatura_atual: 24.0,
        vento_kmh: 10.0,
        limite_temperatura_critica: 35.0,
    };

    let costeira = EstacaoCosteira {
        id: 2,
        nome_local: "Baía de Paranaguá".to_string(),
        latitude: -25.52,
        longitude: -48.51,
        temperatura_atual: 27.0,
        vento_kmh: 60.0,
        limite_vento_critico: 68.0,
    };

    let montanha = EstacaoMontanha {
        id: 3,
        nome_local: "Pico Paraná".to_string(),
        latitude: -25.28,
        longitude: -48.75,
        altitude_m: 1877.0,
        temperatura_atual: 8.0,
        vento_kmh: 22.0,
        limite_temperatura_congelamento: 0.0,
    };

    println!("=== Testando adicionar_estacao ===");
    central.adicionar_estacao(Arc::new(superficie));
    central.adicionar_estacao(Arc::new(costeira));
    central.adicionar_estacao(Arc::new(montanha));
    println!("Total de estações cadastradas: {}\n", central.estacoes.len());

    // ── 4. Testando registrar_leitura — sucesso simples ───────────────
    println!("=== Testando registrar_leitura (estação #1) ===");
    match central.registrar_leitura(1, 1) {
        Ok(leitura) => println!("✓ Leitura criada: {:?}", leitura),
        Err(e) => println!("✗ ERRO inesperado: {:?}", e),
    }

    // ── 5. Testando registrar_leitura — caso crítico (estação #2) ─────
    println!("\n=== Testando registrar_leitura crítico (estação #2 - vento alto) ===");
    match central.registrar_leitura(2, 2) {
        Ok(leitura) => {
            println!("Leitura: {:?}", leitura);
            if leitura.critico {
                println!("✓ Leitura corretamente marcada como crítica");
            } else {
                println!("⚠ Esperava crítico=true (vento 68 > limite 60), veio false");
            }
        }
        Err(e) => println!("✗ ERRO inesperado: {:?}", e),
    }

    // ── 6. Verificando se o alerta foi de fato criado ─────────────────
    println!("\n=== Verificando Vec alertas após leitura crítica ===");
    println!("Total de alertas: {}", central.alertas.len());
    for a in &central.alertas {
        println!("  {:?}", a);
    }
    if central.alertas.is_empty() {
        println!("⚠ Nenhum alerta gerado — verifique se e_critico() e o push em alertas estão corretos");
    }

    // ── 7. Testando registrar_leitura — estação inexistente ───────────
    println!("\n=== Testando registrar_leitura (estação inexistente) ===");
    match central.registrar_leitura(999, 999) {
        Ok(_) => println!("✗ ERRO: deveria ter falhado para estação inexistente"),
        Err(e) => println!("✓ Corretamente rejeitado: {:?}", e),
    }

    // ── 8. Registrando leitura da estação #3 também ───────────────────
    central.registrar_leitura(3, 3).unwrap();
    println!("\nTotal de leituras após os testes manuais: {}", central.leituras.len());

    // ── 9. Testando varrer_rede() ──────────────────────────────────────
    println!("\n=== Testando varrer_rede() ===");
    let novas = central.varrer_rede();
    println!("varrer_rede() retornou {} leituras nesta chamada", novas.len());
    println!("Total acumulado em central.leituras: {}", central.leituras.len());

    if novas.len() != central.estacoes.len() {
        println!(
            "⚠ ATENÇÃO: varrer_rede() deveria retornar {} leituras (uma por estação), retornou {}",
            central.estacoes.len(), novas.len()
        );
    } else {
        println!("✓ Quantidade correta — uma leitura por estação");
    }

    // ── 10. Testando estacoes_criticas() ───────────────────────────────
    println!("\n=== Testando estacoes_criticas() ===");
    let criticas = central.estacoes_criticas();
    println!("Estações em estado crítico: {}", criticas.len());
    for e in &criticas {
        println!("  [{}] {} — vento: {:.1} km/h, temp: {:.1}°C", e.id(), e.nome_local(), e.vento(), e.temperatura());
    }
    if criticas.is_empty() {
        println!("⚠ Esperava ao menos a estação costeira (#2) como crítica");
    }

    // ── 11. Testando leituras_por_estacao(estacao_id) ──────────────────
    println!("\n=== Testando leituras_por_estacao(2) ===");
    let leituras_2 = central.leituras_por_estacao(2);
    println!("Leituras da estação #2: {}", leituras_2.len());
    for l in &leituras_2 {
        println!("  temp: {:.1}, vento: {:.1}, ciclo: {}", l.temperatura, l.vento_kmh, l.ciclo);
    }
    // Registramos manualmente + varrer_rede — deveria ter ao menos 2
    if leituras_2.len() < 2 {
        println!("⚠ Esperava pelo menos 2 leituras para a estação #2, achou {}", leituras_2.len());
    }

    println!("\n=== Testando leituras_por_estacao(estação inexistente) ===");
    let leituras_vazio = central.leituras_por_estacao(999);
    if leituras_vazio.is_empty() {
        println!("✓ Corretamente retornou Vec vazio");
    } else {
        println!("✗ ERRO: deveria ter retornado vazio");
    }

    // ── 12. Testando estacao_mais_quente() ──────────────────────────────
    println!("\n=== Testando estacao_mais_quente() ===");
    match central.estacao_mais_quente() {
        Some(leitura) => {
            println!("Leitura mais quente: {:?}", leitura);
            // Critério: deveria ser a estação #2 (27°C), a mais quente das três
            if leitura.estacao_id == 2 {
                println!("✓ Estação correta identificada (costeira, 27°C)");
            } else {
                println!("⚠ Esperava estacao_id=2 (a mais quente cadastrada), veio {}", leitura.estacao_id);
            }
        }
        None => println!("⚠ Nenhuma leitura encontrada — inesperado neste ponto do teste"),
    }

    // ── 13. Testando gerar_relatorio(tipo, ciclo) — os três formatos ────
    println!("\n=== Testando gerar_relatorio — Resumido ===");
    let rel_resumido = central.gerar_relatorio(TipoRelatorio::Resumido, central.ciclo_atual);
    println!("Título: {}", rel_resumido.titulo_conteudo());
     */

    /*
    // ── 1. Montar uma Central com 3 estações de tipos diferentes ─────
    let superficie = EstacaoSuperficie {
        id: 1,
        nome_local: "Curitiba - Centro".to_string(),
        latitude: -25.42,
        longitude: -49.27,
        temperatura_atual: 24.0,
        vento_kmh: 10.0,
        limite_temperatura_critica: 35.0,
    };

    let costeira = EstacaoCosteira {
        id: 2,
        nome_local: "Baía de Paranaguá".to_string(),
        latitude: -25.52,
        longitude: -48.51,
        temperatura_atual: 27.0,
        vento_kmh: 68.0,
        limite_vento_critico: 60.0,
    };

    let montanha = EstacaoMontanha {
        id: 3,
        nome_local: "Pico Paraná".to_string(),
        latitude: -25.28,
        longitude: -48.75,
        altitude_m: 1877.0,
        temperatura_atual: 8.0,
        vento_kmh: 22.0,
        limite_temperatura_congelamento: 0.0,
    };

    let central = estacao_central {
        estacoes: vec![
            Arc::new(superficie),
            Arc::new(costeira),
            Arc::new(montanha),
        ],
        leituras: Vec::new(),
        alertas: Vec::new(),
        ciclo_atual: 1,
    };

    let total_estacoes = central.estacoes.len();
    let ciclos: u32 = 5;

    // ── 2. Chamar a coleta — se houver deadlock, trava aqui ───────────
    println!("Iniciando coleta com {} estações × {} ciclos...", total_estacoes, ciclos);
    let resultado = central.iniciar_ciclo_coleta(ciclos);
    println!("Coleta concluída — sem deadlock.\n");

    // ── 3. Critério 1 — quantidade total exata ────────────────────────
    let esperado = total_estacoes * ciclos as usize;
    println!("=== Critério 1: quantidade total ===");
    println!("Esperado: {} leituras ({} estações × {} ciclos)", esperado, total_estacoes, ciclos);
    println!("Recebido: {} leituras", resultado.len());

    if resultado.len() == esperado {
        println!("✓ Quantidade exata — nenhuma mensagem perdida.\n");
    } else if resultado.len() < esperado {
        println!("✗ FALTAM {} leituras — verifique se join() está sendo chamado em TODOS os handles antes do recv esvaziar.\n", esperado - resultado.len());
    } else {
        println!("✗ SOBRARAM {} leituras — verifique o range `0..ciclos`.\n", resultado.len() - esperado);
    }
    */

/*    let openmeteo = RespostaOpenMeteo::novo();
    let resposta = openmeteo.buscar_clima(-25.42, 49.27);
    if let Ok(resp) = resposta{
        println!("{:?}", resp);
    }*/

    let superficie = EstacaoSuperficie::novo(
        1, "Curitiba - Centro".to_string(), -25.42, -49.27, 0.0, 0.0, 35.0
    );
    let costeira = EstacaoCosteira::novo(
        2, "Baía de Paranaguá".to_string(), -25.52, -48.51, 0.0, 0.0, 60.0
    );
    let montanha = EstacaoMontanha::novo(
        3, "Pico Paraná".to_string(), -25.28, -48.75, 1877.0, 0.0, 0.0, 0.0
    );

    let estacoes_originais: Vec<Arc<dyn EstacaoMeteorologica>> = vec![
        Arc::new(superficie),
        Arc::new(costeira),
        Arc::new(montanha),
    ];

    let openmeteo_api = RespostaOpenMeteo::novo();
    println!("qtde esperada: {}", estacoes_originais.len());
    let servico_api = openmeteo_api.atualizar_estacoes_da_api(estacoes_originais);
    if let Ok(x) = servico_api {
        println!("retorno: {}", x);
    }

}
