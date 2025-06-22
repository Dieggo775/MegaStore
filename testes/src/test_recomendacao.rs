use my_rust_project::{produtos.csv, recomendador::recomendar_produtos};

#[test]
fn test_recomendacao_simples() {
    let produtos = produtos.csv("produtos_grande.csv").expect("Erro ao carregar CSV");
    let produto = &produtos[1000];
    let recomendacoes = produtos(produto, &produtos, 3);
    assert!(!recomendacoes.is_empty(), "Deveria haver recomendações para o produto");
}
