use std::{error::Error, fs::File};// Importa o trait Error para lidar com erros genéricos e a struct File para abrir arquivos.

use csv::ReaderBuilder;// Importa o ReaderBuilder da crate csv, que permite configurar e criar um leitor CSV.

use crate::sistema_busca::Produto;// Importa a struct Produto do módulo sistema_busca, que representa um produto.

pub fn carregar_produtos_csv(caminho: &str) -> Result<Vec<Produto>, Box<dyn Error>> {
// Declara uma função pública que recebe o caminho de um arquivo CSV como &str
// e retorna um Result com um vetor de Produtos ou um erro genérico.

    let arquivo = File::open(caminho)?;// Tenta abrir o arquivo no caminho informado, retornando erro se falhar.

    let mut leitor = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(arquivo);
// Cria um leitor CSV configurado para reconhecer que o arquivo tem cabeçalho (has_headers(true)).
// O leitor é construído a partir do arquivo aberto.

    let mut produtos = Vec::new();// Inicializa um vetor mutável vazio para armazenar os produtos lidos.

    for resultado in leitor.deserialize() {// Itera sobre cada registro do CSV, tentando desserializar cada linha em um Produto.
// O método deserialize() automaticamente mapeia os campos do CSV para os campos da struct Produto.

        let produto: Produto = resultado?;// Para cada resultado, tenta obter o Produto. Se houver erro na desserialização, ele é propagado com '?'.

        produtos.push(produto);// Adiciona o produto desserializado ao vetor de produtos.
    }

    Ok(produtos)// Retorna o vetor de produtos como resultado bem-sucedido da função.
}
