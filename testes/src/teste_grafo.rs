#[cfg(test)]
mod tests {
    use super::*;

    fn test_recomendacoes() {
        let mut grafo = GrafoRecomendacao::new();
        grafo.adicionar_relacao(1, 2);
        grafo.adicionar_relacao(1, 3);

        let recomendacoes = grafo.recomendar(1).unwrap();
        assert!(recomendacoes.contains(&2));
        assert!(recomendacoes.contains(&3));
        assert_eq!(recomendacoes.len(), 2);
    }
}