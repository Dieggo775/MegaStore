pub mod produto;
// Declara o módulo público `produto`. Geralmente, o código desse módulo estará no arquivo `produto.rs`.
// Esse módulo provavelmente contém a definição da struct Produto e suas funcionalidades.

pub mod indice;// Declara o módulo público `indice`. Provavelmente contém o código relacionado ao índice de busca.

pub mod grafo;// Declara o módulo público `grafo`. Esse módulo deve conter a implementação do grafo de recomendação.

pub mod carregar;// Declara o módulo público `carregar`. Provavelmente contém funções para carregar produtos, como do CSV.

pub use produto::Produto;// Reexporta a struct Produto do módulo `produto`, tornando-a acessível diretamente a quem usar este módulo pai.

pub use indice::IndiceBusca;// Reexporta a struct IndiceBusca do módulo `indice`, para facilitar o acesso a ela.

pub use grafo::GrafoRecomendacao;// Reexporta a struct GrafoRecomendacao do módulo `grafo`.

pub use carregar::carregar_produtos_csv;// Reexporta a função carregar_produtos_csv do módulo `carregar`.
