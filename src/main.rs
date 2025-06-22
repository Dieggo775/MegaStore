mod sistema_busca; // Declara o módulo sistema_busca, que contém lógica de busca, produtos, índices e grafo

mod gui;           // Declara o módulo gui, responsável pela interface gráfica do aplicativo

use gui::MegaStoreApp;
// Importa a struct MegaStoreApp do módulo gui para facilitar o uso no main

fn main() -> Result<(), Box<dyn std::error::Error>> {
// Função principal do programa, que pode retornar um erro genérico

    let mut app = MegaStoreApp::new("produtos.csv")?;
// Cria uma nova instância do aplicativo MegaStoreApp, carregando produtos do arquivo "produtos.csv"
// O operador '?' propaga erros caso o arquivo não seja encontrado ou o carregamento falhe

    app.run();
// Executa a interface gráfica do aplicativo, que mantém o programa rodando até o usuário fechar a janela

    Ok(())
// Retorna Ok(()) indicando que o programa terminou sem erros
}
