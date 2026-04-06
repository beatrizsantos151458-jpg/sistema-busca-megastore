use std::collections::HashMap;

struct Produto {
    nome: String,
    preco: f64,
    categoria: String,
}

// função para adicionar produto
fn adicionar_produto(
    produtos: &mut HashMap<String, Produto>,
    chave: &str,
    nome: &str,
    preco: f64,
    categoria: &str,
) {
    produtos.insert(
        chave.to_string(),
        Produto {
            nome: nome.to_string(),
            preco,
            categoria: categoria.to_string(),
        },
    );
}

// função para buscar produto
fn buscar_produto(produtos: &HashMap<String, Produto>, chave: &str) {
    match produtos.get(chave) {
        Some(produto) => {
            println!("\nProduto encontrado:");
            println!("Nome: {}", produto.nome);
            println!("Preço: R$ {}", produto.preco);
            println!("Categoria: {}", produto.categoria);
        }
        None => println!("Produto não encontrado"),
    }
}

// função para listar todos
fn listar_produtos(produtos: &HashMap<String, Produto>) {
    println!("\nLista de produtos:");
    for (chave, produto) in produtos {
        println!("Código: {}", chave);
        println!("Nome: {}", produto.nome);
        println!("Preço: R$ {}", produto.preco);
        println!("Categoria: {}", produto.categoria);
        println!("------------------------");
    }
}

// função para buscar por categoria
fn buscar_por_categoria(produtos: &HashMap<String, Produto>, categoria: &str) {
    println!("\nProdutos na categoria: {}", categoria);

    for produto in produtos.values() {
        if produto.categoria == categoria {
            println!("Nome: {}", produto.nome);
            println!("Preço: R$ {}", produto.preco);
            println!("------------------------");
        }
    }
}

fn main() {
    let mut produtos: HashMap<String, Produto> = HashMap::new();

    adicionar_produto(&mut produtos, "notebook", "Notebook Dell", 3500.0, "Eletrônicos");
    adicionar_produto(&mut produtos, "celular", "iPhone 13", 5000.0, "Eletrônicos");
    adicionar_produto(&mut produtos, "cadeira", "Cadeira Gamer", 1200.0, "Móveis");

    buscar_produto(&produtos, "celular");
    listar_produtos(&produtos);
    buscar_por_categoria(&produtos, "Eletrônicos");
}