mod entidades;
mod agregador;
mod relatorios;
mod erros;

use std::sync::Arc;
use agregador::central::{estacao_central};
use entidades::modelos::{EstacaoSuperficie, EstacaoCosteira, EstacaoMontanha};
use relatorios::modelo_relatorio::RelatorioResumido;
use crate::relatorios::modelo_relatorio::TipoRelatorio;

fn main() {

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

}
