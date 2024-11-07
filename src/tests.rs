#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cadastrar_livro() {
        let mut biblioteca = BibliotecaLivros::novo();
        assert_eq!(biblioteca.cadastrar_livro("001", "O Hobbit", "J.R.R. Tolkien", 1937), Ok(()));
    }

    #[test]
    fn test_remover_livro() {
        let mut biblioteca = BibliotecaLivros::novo();
        biblioteca.cadastrar_livro("001", "O Hobbit", "J.R.R. Tolkien", 1937).unwrap();
        assert_eq!(biblioteca.remover_livro("001"), Ok(()));
    }
}
