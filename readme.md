# 🚀 MegaStore - Motor de Busca em Grafos (Rust)

Este é um projeto acadêmico desenvolvido em **Rust** para solucionar problemas de busca e recomendação de produtos em larga escala. O motor utiliza uma estrutura de **Grafo** para conectar produtos, categorias e marcas, permitindo recomendações inteligentes baseadas em relacionamentos (arestas).

## 📌 O Problema
A MegaStore identificou que sistemas de busca tradicionais (lineares) são lentos e imprecisos para catálogos com milhões de itens. O objetivo deste projeto é fornecer uma solução:
- **Rápida:** Busca eficiente em grafos.
- **Escalável:** Persistência binária de alta performance.
- **Inteligente:** Sugestões automáticas baseadas nas conexões do catálogo.

## ✨ Funcionalidades
- **Estrutura de Grafo:** Representação de dados através de Nós (Produtos/Categorias) e Arestas (Conexões).
- **Busca por Nome:** Localização instantânea de itens no catálogo.
- **Recomendações Inteligentes:** Ao buscar um produto, o motor sugere automaticamente itens conectados no grafo (ex: buscar um Celular recomenda Carregadores e a Categoria de Eletrônicos).
- **Persistência Binária:** O catálogo é salvo em um arquivo `.bin` (formato Bincode), garantindo carregamento ultrarrápido.
- **CLI Interativo:** Interface via terminal simples e intuitiva.

## 🛠️ Tecnologias Utilizadas
- **Linguagem:** [Rust](https://www.rust-lang.org/) (Edição 2024).
- **Serialização:** [Serde](https://serde.rs/) & [Bincode](https://github.com/bincode-org/bincode).
- **Estrutura de Dados:** Grafos (Lista de Adjacência).

## 📂 Estrutura do Projeto
```text
├── src/
│   ├── main.rs      # Interface CLI e ponto de entrada.
│   ├── models.rs    # Definição dos modelos (Nó, Tipo de Nó, Grafo).
│   └── engine.rs    # Lógica do motor de busca e persistência.
├── data/
│   └── grafo_store.bin # Arquivo de dados persistidos.
└── Cargo.toml       # Dependências e metadados do projeto.
```

## 🚀 Como Executar

### Pré-requisitos
- Ter o **Rust** instalado ([instruções aqui](https://www.rust-lang.org/tools/install)).

### Passo a Passo
1. Clone este repositório:
   ```bash
   git clone https://github.com/cassio8732/megastore.git
   cd megastore
   ```

2. Execute o projeto:
   ```bash
   cargo run
   ```

3. No terminal, digite o nome de um produto (ex: `Samsung`, `iPhone`, `Sony`) para ver a busca e as recomendações em ação. Digite `sair` para encerrar.

## 🧠 Conceitos Aplicados (Acadêmico)
- **Vértices (Nós):** Cada produto ou categoria é um vértice no grafo.
- **Arestas (Conexões):** Definem a relação entre os itens (ex: Produto X pertence à Categoria Y).
- **Complexidade de Espaço:** Otimizada através da persistência binária, evitando o uso de bancos de dados pesados.

---
Desenvolvido como projeto acadêmico para o motor de busca da **MegaStore**.
