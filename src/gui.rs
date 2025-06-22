// Importações da biblioteca FLTK (para interface gráfica)
use fltk::{
    app, // Controla o ciclo principal da aplicação
    button::Button, // Botão clicável
    browser::HoldBrowser, // Lista interativa para exibir resultados
    enums::{Align, Color}, // Alinhamentos e cores
    frame::Frame, // Elemento de texto estático (título, rótulo)
    group::Flex, // Container flexível (layout responsivo em linha/coluna)
    input::Input, // Campo de entrada de texto
    prelude::*, // Traits necessários para usar os widgets
    text::{TextBuffer, TextDisplay}, // Widgets para exibir textos não editáveis
    window::Window, // Janela principal
};

// Biblioteca para normalizar strings e remover acentos
use unicode_normalization::UnicodeNormalization;

// Módulos locais do projeto
use crate::sistema_busca::{Produto, IndiceBusca, GrafoRecomendacao, carregar_produtos_csv};
use crate::recomendador::{Recomendacao, recomendar_produtos}; // Módulo que gera recomendações

// Função auxiliar para identificar se o caractere é um acento/diacrítico
fn is_mark(c: char) -> bool {
    matches!(
        c,
        '\u{0300}'..='\u{036F}'   // Faixa padrão de acentos
        | '\u{1AB0}'..='\u{1AFF}'
        | '\u{1DC0}'..='\u{1DFF}'
        | '\u{20D0}'..='\u{20FF}'
        | '\u{FE20}'..='\u{FE2F}'
    )
}

// Função que remove acentos de uma string usando normalização
fn remover_acentos(s: &str) -> String {
    s.nfd().filter(|c| !is_mark(*c)).collect()
}

// Estrutura principal da aplicação
#[derive(Clone)]
pub struct MegaStoreApp {
    indice: IndiceBusca, // Índice de busca eficiente
    grafo: GrafoRecomendacao, // Estrutura de grafo para gerar recomendações
    produtos: Vec<Produto>, // Lista de produtos carregados
}

impl MegaStoreApp {
    // Construtor: carrega produtos do CSV e inicializa o índice e grafo
    pub fn new(caminho_csv: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let produtos = carregar_produtos_csv(caminho_csv)?; // Carrega CSV
        let mut indice = IndiceBusca::new(); // Novo índice de busca
        let mut grafo = GrafoRecomendacao::new(); // Novo grafo

        for produto in &produtos {
            indice.adicionar_produto(produto.clone()); // Indexa produto
            grafo.adicionar_produto(produto.id); // Adiciona ao grafo
        }

        Ok(MegaStoreApp { indice, grafo, produtos })
    }

    // Função principal que inicia a interface e lógica da aplicação
    pub fn run(&mut self) {
        let app = app::App::default(); // Inicia app FLTK

        // Cria a janela principal
        let mut wind = Window::new(100, 100, 700, 750, "🛍️ MegaStore - Busca com Filtros");
        wind.set_color(Color::from_rgb(240, 240, 240)); // Cor de fundo
        wind.make_resizable(true); // Permite redimensionar
        let mut wind = wind.center_screen(); // Centraliza na tela

        // Container principal empilhado verticalmente
        let mut container = Flex::default_fill().column();
        container.set_margin(100); // Margem externa
        container.set_pad(10);     // Espaçamento entre widgets

        // Título da aplicação
        let mut titulo = Frame::default()
            .with_label("🛍️ MegaStore")
            .with_align(Align::Center);
        titulo.set_label_size(45);
        container.fixed(&titulo, 100); // Altura fixa do título

        // Campo de busca por nome
        let mut input_busca = Input::default().with_label("🔍 Buscar:");
        input_busca.set_value("Digite o nome do produto");
        input_busca.set_text_color(Color::from_rgb(160, 160, 160)); // Texto inicial em cinza claro

        // Limpa texto ao focar no campo
        {
            let mut input_clone = input_busca.clone();
            input_busca.set_callback(move |_| {
                if input_clone.value() == "Digite o nome do produto" {
                    input_clone.set_value("");
                    input_clone.set_text_color(Color::Black); // Muda cor ao digitar
                }
            });
        }
        container.fixed(&input_busca, 60); // Altura fixa do campo

        // Filtros adicionais: categoria e marca
        let mut filtros_flex = Flex::default().row();
        filtros_flex.set_pad(100); // Espaço entre filtros
        filtros_flex.set_align(Align::Center);

        let input_categoria = Input::default().with_label("📂 Categoria:");
        filtros_flex.fixed(&input_categoria, 200); // Largura fixa

        let input_marca = Input::default().with_label("🏷️ Marca:");
        filtros_flex.fixed(&input_marca, 200);

        filtros_flex.end();
        container.fixed(&filtros_flex, 50); // Altura do bloco de filtros

        // Botão de busca
        let mut botao_buscar = Button::default().with_label("🔎 Buscar");
        botao_buscar.set_color(Color::from_rgb(70, 130, 180)); // Cor azul
        botao_buscar.set_label_color(Color::White); // Texto branco
        container.fixed(&botao_buscar, 45);

        // Lista dos produtos encontrados
        let mut browser_resultados = HoldBrowser::default();
        container.fixed(&browser_resultados, 100);

        // Título da seção de recomendações
        let mut titulo_rec = Frame::default().with_label("🔗 Recomendações:");
        titulo_rec.set_label_size(20);
        container.fixed(&titulo_rec, 30);

        // Área onde as recomendações serão exibidas
        let mut display_recomendacoes = TextDisplay::default();
        let buffer_recomendacoes = TextBuffer::default();
        display_recomendacoes.set_buffer(Some(buffer_recomendacoes.clone()));
        container.fixed(&display_recomendacoes, 150);

        container.end(); // Finaliza layout
        wind.end(); // Finaliza janela
        wind.show(); // Exibe na tela

        // Clones de dados para uso nos callbacks
        let produtos = self.produtos.clone();

        // Lógica do botão de busca
        botao_buscar.set_callback({
            let produtos = produtos.clone();
            let mut browser_resultados = browser_resultados.clone();
            let input_busca = input_busca.clone();
            let input_categoria = input_categoria.clone();
            let input_marca = input_marca.clone();

            move |_| {
                // Processa os filtros com acento removido e minúsculo
                let nome_query = remover_acentos(&input_busca.value().to_lowercase());
                let filtro_cat = remover_acentos(&input_categoria.value().to_lowercase());
                let filtro_mar = remover_acentos(&input_marca.value().to_lowercase());

                let mut resultados: Vec<&Produto> = produtos.iter().collect();

                // Filtra por nome
                if !nome_query.is_empty() {
                    resultados = resultados
                        .into_iter()
                        .filter(|p| remover_acentos(&p.nome.to_lowercase()).contains(&nome_query))
                        .collect();
                }

                // Filtra por categoria
                if !filtro_cat.is_empty() {
                    resultados = resultados
                        .into_iter()
                        .filter(|p| remover_acentos(&p.categoria.to_lowercase()).contains(&filtro_cat))
                        .collect();
                }

                // Filtra por marca
                if !filtro_mar.is_empty() {
                    resultados = resultados
                        .into_iter()
                        .filter(|p| remover_acentos(&p.marca.to_lowercase()).contains(&filtro_mar))
                        .collect();
                }

                // Atualiza a lista de resultados
                browser_resultados.clear();
                for prod in &resultados {
                    browser_resultados.add(&format!("{} | {} | {}", prod.nome, prod.categoria, prod.marca));
                }
            }
        });

        // Quando usuário clica em um item da lista, exibe recomendações
        browser_resultados.set_callback({
            let produtos = produtos.clone();
            let mut buffer_recomendacoes = buffer_recomendacoes.clone();

            move |b| {
                let idx = b.value(); // Índice selecionado
                if idx == 0 {
                    buffer_recomendacoes.set_text(""); // Limpa se nada foi selecionado
                    return;
                }

                let idx = (idx - 1) as usize;

                if let Some(produto_selecionado) = produtos.get(idx) {
                    // Obtém produtos recomendados
                    let recomendados: Vec<Recomendacao> = recomendar_produtos(produto_selecionado, &produtos, 3);

                    // Converte recomendações em texto
                    let texto_rec = if recomendados.is_empty() {
                        "Nenhuma recomendação encontrada.".to_string()
                    } else {
                        recomendados.iter()
                            .map(|rec| {
                                format!(
                                    "🛒 {} | {} | {} (pontuação: {})",
                                    rec.produto.nome, rec.produto.categoria, rec.produto.marca, rec.pontuacao
                                )
                            })
                            .collect::<Vec<_>>()
                            .join("\n")
                    };

                    // Mostra no display
                    buffer_recomendacoes.set_text(&texto_rec);
                }
            }
        });

        app.run().unwrap(); // Inicia o loop da aplicação
    }
}
