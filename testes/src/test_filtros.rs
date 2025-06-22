use my_rust_project::produtos.csv;

#[test]
fn test_filtrar_por_categoria() {
    let produtos = produtos.csv("produtos_grande.csv").expect("Erro ao carregar CSV");
    let categoria = "Categoria 10".to_lowercase();
    let filtrados: Vec<_> = produtos.iter()
        .filter(|p| p.categoria.to_lowercase().contains(&categoria))
        .collect();
    assert!(!filtrados.is_empty(), "Deveria haver produtos da Categoria 10");
}

#[test]
fn test_filtrar_por_marca() {
    let produtos = produtos.csv("produtos_grande.csv").expect("Erro ao carregar CSV");
    let marca = "Marca 20".to_lowercase();
    let filtrados: Vec<_> = produtos.iter()
        .filter(|p| p.marca.to_lowercase().contains(&marca))
        .collect();
    assert!(!filtrados.is_empty(), "Deveria haver produtos da Marca 20");
}
