use std::fs::File;
use std::io::{Read, Write};
use crate::models::{Grafo, No};

pub struct MotorBusca {
    pub grafo: Grafo,
}

impl MotorBusca {
    pub fn novo() -> Self {
        MotorBusca {
            grafo: Grafo::novo(),
        }
    }

    pub fn adicionar_no(&mut self, no: No) {
        self.grafo.nos.push(no);
    }

    pub fn buscar_por_nome(&self, termo: &str) -> Option<(&No, Vec<&No>)> {
        let no_encontrado = self.grafo.nos.iter().find(|n| n.nome.to_lowercase().contains(&termo.to_lowercase()));

        if let Some(no) = no_encontrado {
            let recomendacoes: Vec<&No> = no.conexoes.iter()
                .filter_map(|&id| self.grafo.nos.get(id as usize))
                .collect();
            
            Some((no, recomendacoes))
        } else {
            None
        }
    }

    pub fn salvar_em_arquivo(&self, caminho: &str) -> std::io::Result<()> {
        let codificado: Vec<u8> = bincode::serialize(&self.grafo).expect("Falha ao serializar");
        let mut arquivo = File::create(caminho)?;
        arquivo.write_all(&codificado)?;
        Ok(())
    }

    pub fn carregar_do_arquivo(&mut self, caminho: &str) -> std::io::Result<()> {
        let mut arquivo = File::open(caminho)?;
        let mut buffer = Vec::new();
        arquivo.read_to_end(&mut buffer)?;
        self.grafo = bincode::deserialize(&buffer).expect("Falha ao deserializar");
        Ok(())
    }
}
