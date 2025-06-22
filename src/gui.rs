// Importa os módulos do FLTK para construir a interface gráfica
use fltk::{
    app,                        // Gerencia o ciclo de vida da aplicação
    button::Button,             // Componente de botão
    enums::{Align, Color},      // Enumerações para alinhamento e cores
    frame::Frame,               // Componente de texto fixo (como título)
    group::Flex,                // Componente de layout flexível (colunas ou linhas)
    input::Input,               // Campo de entrada de texto
    prelude::*,                 // Traits comuns para componentes (como .set_label, .with_label etc)
    text::{TextBuffer, TextDisplay}, // Área de texto para exibir resultados
    window::Window,             // Janela principal
};

// Para normalizar e remover acentos
use unicode_normalization::UnicodeNormalization;

// Importa as estruturas do sistema de busca
use crate::sistema_busca::{Produto, IndiceBusca, GrafoRecomendacao, carregar_produtos_csv};

// Função auxiliar para detectar caracteres de acento (combining marks)
fn is_mark(c: char) -> bool {
    matches!(
        c,
        '\u{0300}'..='\u{036F}'     // Acentos latinos
        | '\u{1AB0}'..='\u{1AFF}'   // Marcas de extensão
        | '\u{1DC0}'..='\u{1DFF}'   // Pequenas marcas adicionais
        | '\u{20D0}'..='\u{20FF}'   // Marcas de posição
        | '\u{FE20}'..='\u{FE2F}'   // Marcas de compatibilidade
    )
}

// Remove acentos de uma string, útil para busca
fn remover_acentos(s: &str) -> String {
    s.nfd().filter(|c| !is_mark(*c)).collect()
}

// Estrutura principal do app
#[derive(Clone)]
pub struct MegaStoreApp {
    indice: IndiceBusca,             // Índice invertido para buscas rápidas
    grafo: GrafoRecomendacao,       // Grafo para recomendações (futuro)
    produtos: Vec<Produto>,         // Lista de produtos carregada do CSV
}

impl MegaStoreApp {
    // Construtor: lê o CSV, preenche os índices e o grafo
    pub fn new(caminho_csv: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let produtos = carregar_produtos_csv(caminho_csv)?;
        let mut indice = IndiceBusca::new();
        let mut grafo = GrafoRecomendacao::new();

        // Adiciona cada produto ao índice e ao grafo
        for produto in &produtos {
            indice.adicionar_produto(produto.clone());
            grafo.adicionar_produto(produto.id);
        }

        Ok(MegaStoreApp {
            indice,
            grafo,
            produtos,
        })
    }

    // Função principal que roda a interface
    pub fn run(&mut self) {
        let app = app::App::default(); // Inicia o ciclo de vida da aplicação
        let mut wind = Window::new(100, 100, 700, 750, "🛍️ MegaStore - Busca com Filtros");
        wind.set_color(Color::from_rgb(240, 240, 240)); // Define cor de fundo
        wind.make_resizable(true);                      // Torna a janela redimensionável
        let mut wind = wind.center_screen();            // Centraliza na tela

        // Container vertical centralizado
        let mut container = Flex::default_fill().column();
        container.set_margin(100); // Margem ao redor
        container.set_pad(10);     // Espaço entre elementos
        container.set_align(Align::Center);

        // Título do app
        let mut titulo = Frame::default()
            .with_label("🛍️ MegaStore")
            .with_align(Align::Center);
        titulo.set_label_size(45); // Tamanho da fonte
        container.fixed(&titulo, 100); // Altura fixa

        // Campo de entrada da busca
        let mut input_busca = Input::default().with_label("🔍 Buscar:");
        input_busca.set_value("Digite o nome do produto"); // Placeholder
        input_busca.set_text_color(Color::from_rgb(160, 160, 160)); // Cinza claro
        

        // Quando o campo ganha foco, limpa o placeholder
        {
            let mut input_clone = input_busca.clone();
            input_busca.set_callback(move |_| {
                if input_clone.value() == "Digite o nome do produto" {
                    input_clone.set_value("");
                    input_clone.set_text_color(Color::Black); // Texto preto
                }
            });
        }
        container.fixed(&input_busca, 40);

        // Layout horizontal para filtros
        let mut filtros_flex = Flex::default().row();
        filtros_flex.set_pad(100);
        filtros_flex.set_align(Align::Center);

        // Campo de categoria
        let input_categoria = Input::default().with_label("📂 Categoria:");
        filtros_flex.fixed(&input_categoria, 200);

        // Campo de marca
        let input_marca = Input::default().with_label("🏷️ Marca:");
        filtros_flex.fixed(&input_marca, 200);

        filtros_flex.end(); // Finaliza o Flex dos filtros
        container.fixed(&filtros_flex, 50);

        // Botão de busca
        let mut botao_buscar = Button::default().with_label("🔎 Buscar");
        botao_buscar.set_color(Color::from_rgb(70, 130, 180)); // Azul SteelBlue
        botao_buscar.set_label_color(Color::White);            // Texto branco
        container.fixed(&botao_buscar, 45);

        // Área de exibição dos resultados
        let mut display_resultados = TextDisplay::default();
        let mut buffer_resultados = TextBuffer::default();
        display_resultados.set_buffer(Some(buffer_resultados.clone()));
        container.fixed(&display_resultados, 350); // Tamanho flexível

        container.end(); // Fecha o container principal
        wind.end();      // Fecha a janela
        wind.show();     // Exibe na tela

        // Clona os produtos para uso no botão
        let produtos = self.produtos.clone();

        // Define a lógica de busca quando o botão é clicado
        botao_buscar.set_callback(move |_| {
            // Normaliza as entradas
            let nome_query = remover_acentos(&input_busca.value().to_lowercase());
            let filtro_cat = remover_acentos(&input_categoria.value().to_lowercase());
            let filtro_mar = remover_acentos(&input_marca.value().to_lowercase());

            // Começa com todos os produtos
            let mut resultados: Vec<&Produto> = produtos.iter().collect();

            // Aplica filtro por nome se houver
            if !nome_query.is_empty() {
                resultados = resultados
                    .into_iter()
                    .filter(|p| remover_acentos(&p.nome.to_lowercase()).contains(&nome_query))
                    .collect();
            }

            // Filtro por categoria (opcional)
            if !filtro_cat.is_empty() {
                resultados = resultados
                    .into_iter()
                    .filter(|p| remover_acentos(&p.categoria.to_lowercase()).contains(&filtro_cat))
                    .collect();
            }

            // Filtro por marca (opcional)
            if !filtro_mar.is_empty() {
                resultados = resultados
                    .into_iter()
                    .filter(|p| remover_acentos(&p.marca.to_lowercase()).contains(&filtro_mar))
                    .collect();
            }

            // Gera texto do resultado
            let resultado_texto = if resultados.is_empty() {
                "⚠️ Nenhum produto encontrado com esses filtros.".to_string()
            } else {
                resultados
                    .iter()
                    .map(|prod| {
                        format!(
                            "🛒 ID: {} - {} | {} | {}\n",
                            prod.id, prod.nome, prod.categoria, prod.marca
                        )
                    })
                    .collect()
            };

            // Exibe na tela
            buffer_resultados.set_text(&resultado_texto);
        });

        // Roda a aplicação
        app.run().unwrap();
    }
}
