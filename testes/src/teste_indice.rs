use sistema_busca::produto::Produto;
use sistema_busca::indice::IndiceBusca;

fn produtos_exemplo() -> Vec<Produto> {
    vec![
        Produto { id: 1, nome: "Notebook Gamer".into(), categoria: "Eletrônicos".into(), marca: "MarcaX".into() },
        Produto { id: 2, nome: "Mouse Gamer".into(), categoria: "Eletrônicos".into(), marca: "MarcaY".into() },
        Produto { id: 3, nome: "Teclado Mecânico".into(), categoria: "Eletrônicos".into(), marca: "MarcaZ".into() },
        Produto { id: 4, nome: "Camiseta Esportiva".into(), categoria: "Vestuário".into(), marca: "MarcaD".into() },
    ]
}

#[test]
fn test_busca_and() {
    let produtos = produtos_exemplo();
    let mut indice = IndiceBusca::new();
    for p in &produtos {
        indice.inserir_produto(p);
    }

    let resultado = indice.buscar(&["gamer", "notebook"]);
    assert_eq!(resultado, vec![1].into_iter().collect());
}

#[test]
fn test_busca_or() {
    let produtos = produtos_exemplo();
    let mut indice = IndiceBusca::new();
    for p in &produtos {
        indice.inserir_produto(p);
    }

    let resultado = indice.buscar_qualquer(&["gamer", "mecânico"]);
    assert_eq!(resultado, vec![1, 2, 3].into_iter().collect());
}
