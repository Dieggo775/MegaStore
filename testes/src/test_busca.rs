use my_rust_project::{produtos.csv, IndiceBusca};

#[test]
fn test_busca_por_nome() {
    let produtos = produtos.csv("produtos_grande.csv").expect("Erro ao carregar CSV");
    let mut indice = IndiceBusca::new();
    for p in &produtos {
        indice.adicionar_produto(p.clone());
    }

    let resultado = indice.buscar_por_nome("Produto 2500");
    assert!(!resultado.is_empty(), "Produto 2500 deveria existir na busca");
}
