use crate::sistema_busca::Produto;

// Representa um item recomendado com a pontuacao
#[derive(Clone)]
pub struct Recomendacao {
    pub produto: Produto,
    pub pontuacao: u32,
}

// Funcao que gera recomendacoes com base em um produto pesquisado
pub fn recomendar_produtos(produto: &Produto, todos: &[Produto], limites: usize) -> Vec<Recomendacao> {
    let mut recomendacoes: Vec<Recomendacao> = todos
        .iter()
        .filter(|p| p.id != produto.id) // Nao recomenda ele mesmo
        .map(|p| {
            let mut pontuacao = 0;
            if p.categoria == produto.categoria {
                pontuacao += 1;
            }
            if p.marca == produto.marca {
                pontuacao += 1;
            }
            Recomendacao {
                produto: p.clone(),
                pontuacao,
            }
        })
        .filter(|r| r.pontuacao > 0) // So recomenda se houver mais de uma pesquisa
        .collect();

        //Ordena por maior pontuacao
        recomendacoes.sort_by_key(|r| std::cmp::Reverse(r.pontuacao));

        // Limita a quantidade de resultados
        recomendacoes.into_iter().take(limites).collect()
}