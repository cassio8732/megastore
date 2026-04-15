mod models;
mod engine;

use models::{No, TipoNo};
use engine::MotorBusca;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    println!("==============================================");
    println!("   MegaStore - Motor de Busca em Grafos      ");
    println!("==============================================");
    
    let mut motor = MotorBusca::novo();
    let caminho_dados = "data/grafo_store.bin";

    // 1. Persistência: Carregamento inicial
    if Path::new(caminho_dados).exists() {
        println!("[INFO] Carregando catálogo de: {}...", caminho_dados);
        if let Err(e) = motor.carregar_do_arquivo(caminho_dados) {
            eprintln!("[ERRO] Falha na persistência: {}", e);
        }
    } else {
        println!("[INFO] Inicializando novo catálogo de exemplo...");
        gerar_dados_exemplo(&mut motor);
        if let Err(e) = motor.salvar_em_arquivo(caminho_dados) {
            eprintln!("[ERRO] Falha ao salvar: {}", e);
        }
    }

    // 2. Terminal Interativo (Busca e Recomendação)
    loop {
        print!("\nDigite o nome do produto (ou 'sair'): ");
        io::stdout().flush().unwrap();

        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Falha ao ler comando");
        let termo_busca = entrada.trim();

        if termo_busca.to_lowercase() == "sair" {
            println!("Encerrando Motor MegaStore. Até logo!");
            break;
        }

        if termo_busca.is_empty() {
            continue;
        }

        // Realiza a busca no grafo
        match motor.buscar_por_nome(termo_busca) {
            Some((no, recomendacoes)) => {
                println!("\n>> Produto Encontrado: {} (ID: {})", no.nome, no.id);
                println!(">> Tipo: {:?}", no.tipo_no);
                println!("----------------------------------------------");
                println!("Sugestões de produtos relacionados (Arestas):");
                
                if recomendacoes.is_empty() {
                    println!("   (Nenhuma recomendação disponível para este item)");
                } else {
                    for rec in recomendacoes {
                        println!("   [+] {} ({:?})", rec.nome, rec.tipo_no);
                    }
                }
                println!("----------------------------------------------");
            }
            None => {
                println!("\n[!] Nenhum produto encontrado com o termo '{}'.", termo_busca);
            }
        }
    }
}

fn gerar_dados_exemplo(motor: &mut MotorBusca) {
    let nos = vec![
        No { id: 0, nome: "Eletrônicos".to_string(), tipo_no: TipoNo::Categoria, conexoes: vec![1, 2, 4] },
        No { id: 1, nome: "Smartphone Samsung Galaxy".to_string(), tipo_no: TipoNo::Produto, conexoes: vec![0, 3] },
        No { id: 2, nome: "Smartphone Apple iPhone".to_string(), tipo_no: TipoNo::Produto, conexoes: vec![0, 3] },
        No { id: 3, nome: "Carregador Ultra Fast".to_string(), tipo_no: TipoNo::Produto, conexoes: vec![1, 2] },
        No { id: 4, nome: "Fone Bluetooth Sony".to_string(), tipo_no: TipoNo::Produto, conexoes: vec![0, 1, 2] },
    ];

    for no in nos {
        motor.adicionar_no(no);
    }
}
