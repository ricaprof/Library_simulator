mod biblioteca;
mod livros;
mod usuarios;
mod emprestimos;

use biblioteca::Biblioteca;

fn main() {
    let mut biblioteca = Biblioteca::new();

    // Exemplo de cadastro de livros
    biblioteca.cadastrar_livro("O Hobbit", "J.R.R. Tolkien", 1937, "12345").unwrap();
    biblioteca.cadastrar_livro("1984", "George Orwell", 1949, "67890").unwrap();

    // Exemplo de busca e listagem
    biblioteca.listar_livros();
    biblioteca.buscar_livro_por_titulo("O Hobbit").unwrap();
    biblioteca.buscar_livro_por_autor("George Orwell").unwrap();
}
