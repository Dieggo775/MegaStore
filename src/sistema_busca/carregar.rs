use crate::sistema_busca::Produto; // Importa a struct Produto do módulo sistema_busca para usar neste arquivo.
use std::error::Error;// Importa o trait Error para usar como tipo de erro genérico na função.
use std::fs::File;// Importa a struct File para abrir arquivos no sistema de arquivos.
use std::io::{BufRead, BufReader};// Importa traits e structs para ler arquivos linha a linha de forma eficiente.

pub fn carregar_produtos_csv(caminho: &str) -> Result<Vec<Produto>, Box<dyn Error>> {
// Declara uma função pública chamada carregar_produtos_csv que recebe um caminho (string) para um arquivo CSV
// e retorna um Result contendo um vetor de Produtos ou um erro genérico.

    let arquivo = File::open(caminho)?;// Tenta abrir o arquivo no caminho passado, retornando um erro se falhar (o '?' propaga o erro).

    let leitor = BufReader::new(arquivo);// Cria um leitor buffered (buf_reader) para o arquivo, que permite ler linha a linha de forma eficiente.

    let mut produtos = Vec::new();// Inicializa um vetor mutável vazio para armazenar os produtos que serão lidos do CSV.

    for linha in leitor.lines() {// Itera sobre cada linha do arquivo usando o método lines() do BufReader.

        let linha = linha?;// Tenta obter a linha atual, propagando erro se houver algum problema na leitura.

        let campos: Vec<&str> = linha.split(',').collect();// Divide a linha em campos separados por vírgula, criando um vetor de strings fatiadas (&str).

        if campos.len() >= 4 {// Verifica se a linha tem pelo menos 4 campos, para garantir que os dados necessários estejam presentes.

            let produto = Produto {
                id: campos[0].parse().unwrap_or(0),// Tenta converter o primeiro campo (id) para um número, usando 0 caso falhe.

                nome: campos[1].to_string(),// Copia o segundo campo para o nome do produto (converte &str para String).

                categoria: campos[2].to_string(),// Copia o terceiro campo para a categoria do produto.

                marca: campos[3].to_string(),// Copia o quarto campo para a marca do produto.
            };

            produtos.push(produto);// Adiciona o produto criado ao vetor de produtos.
        }
    }

    Ok(produtos)// Retorna o vetor de produtos como sucesso da função.
}
