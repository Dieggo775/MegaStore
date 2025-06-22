#[derive(Clone)]// Deriva a trait Clone para a struct Produto, permitindo criar cópias (clones) de instâncias de Produto.

pub struct Produto {// Define uma struct pública chamada Produto, que representa um produto no sistema.

    pub id: u32,// Campo público que armazena o identificador único do produto, do tipo inteiro sem sinal 32 bits.

    pub nome: String,// Campo público que armazena o nome do produto, do tipo String (texto alocado dinamicamente).

    pub categoria: String,// Campo público que armazena a categoria do produto, também como String.

    pub marca: String,// Campo público que armazena a marca do produto, também do tipo String.
}
