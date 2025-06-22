# 🛍️ Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## 📘 Descrição do Projeto

Este projeto implementa um sistema de busca e recomendação de produtos utilizando **A Linguagem Rust** e estruturas de dados eficientes como **Tabelas Hash** e **Grafos**. A aplicação simula o cenário de uma grande empresa de e-commerce, a **MegaStore**, que busca melhorar a precisão e a velocidade das buscas em seu vasto catálogo.

A aplicação inclui uma interface gráfica (GUI) construída com **FLTK**, permitindo buscas por **nome**, **categoria** e **marca**, além de apresentar recomendações inteligentes com base em similaridades entre os produtos.

---

## 🛠️ Tecnologias Utilizadas

- **Rust** (linguagem principal)
- **FLTK** (`fltk = "1"`) - GUI leve e rápida
- **unicode-normalization** (`unicode-normalization = "0.1"`) - para normalização e busca sem acento
- **Cargo** - gerenciador de pacotes e build
- **HashMap**, **Vec**, **Grafos** - estruturas de dados nativas de Rust
- **Modularização** com múltiplos arquivos (gui.rs, sistema_busca.rs, recomendador.rs)

---

## ▶️ Como Executar o Projeto

### 1. Clone o repositório:

```bash
git clone https://github.com/seu-usuario/megastore-rust.git
cd MY_RUST_PROJECT
