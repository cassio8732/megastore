use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TipoNo {
    Produto,
    Marca,
    Categoria,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct No {
    pub id: u32,
    pub nome: String,
    pub tipo_no: TipoNo,
    pub conexoes: Vec<u32>, // IDs dos outros nós conectados (Arestas)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Grafo {
    pub nos: Vec<No>,
}

impl Grafo {
    pub fn novo() -> Self {
        Grafo { nos: Vec::new() }
    }
}
