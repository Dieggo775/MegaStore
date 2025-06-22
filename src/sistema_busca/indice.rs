use crate::sistema_busca::Produto;// Importa a struct Produto do módulo sistema_busca para utilizar neste código.

use std::collections::HashMap;// Importa a estrutura HashMap, que será usada para armazenar um índice de busca.

#[derive(Clone)]// Deriva a trait Clone para a struct IndiceBusca, permitindo copiar instâncias dela.

pub struct IndiceBusca {
    mapa: HashMap<String, Vec<u32>>, // mapa palavra -> lista de ids
    // Declara um HashMap que mapeia uma palavra (String) para uma lista de IDs (u32) de produtos que contêm essa palavra.
}

impl IndiceBusca {
    pub fn new() -> Self {// Função pública para criar um novo índice de busca vazio.

        IndiceBusca {
            mapa: HashMap::new(),// Inicializa o HashMap vazio para armazenar o índice.
        }
    }

    pub fn adicionar_produto(&mut self, produto: Produto) {// Método público que recebe uma referência mutável ao índice e um Produto a ser adicionado.

        for palavra in produto.nome.to_lowercase().split_whitespace() {// Itera sobre cada palavra do nome do produto, convertendo para minúsculas e separando por espaços.

            self.mapa.entry(palavra.to_string()).or_default().push(produto.id);
            // Para cada palavra, acessa a entrada correspondente no HashMap:
            // - Se a palavra ainda não existe, cria uma nova lista vazia (or_default()).
            // - Adiciona o ID do produto à lista associada à palavra.
        }
    }

    pub fn buscar(&self, termo: &str) -> Vec<u32> {// Método público para buscar no índice os produtos que contêm o termo especificado.

        self.mapa.get(termo)// Busca no HashMap a lista de IDs associada ao termo (palavra).

            .cloned()// Clona a lista de IDs para evitar devolver uma referência interna.

            .unwrap_or_default()// Se o termo não existir no mapa, retorna um vetor vazio.
    }
}
