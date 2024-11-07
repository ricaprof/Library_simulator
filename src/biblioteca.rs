use crate::livros::Livros;
use crate::usuarios::Usuarios;
use crate::emprestimos::Emprestimos;
use std::fs;

pub struct Biblioteca {
    livros: Livros,
    usuarios: Usuarios,
    emprestimos: Emprestimos,
}

impl Biblioteca {
    pub fn new() -> Self {
        Biblioteca {
            livros: Livros::new(),
            usuarios: Usuarios::new(),
            emprestimos: Emprestimos::new(),
        }
    }

    pub fn cadastrar_livro(&mut self, titulo: &str, autor: &str, ano: u32, codigo: &str) -> Result<(), String> {
        self.livros.adicionar_livro(titulo, autor, ano, codigo)
    }

    pub fn listar_livros(&self) {
        self.livros.listar_livros();
    }

    pub fn buscar_livro_por_titulo(&self, titulo: &str) -> Result<(), String> {
        self.livros.buscar_por_titulo(titulo)
    }

    pub fn buscar_livro_por_autor(&self, autor: &str) -> Result<(), String> {
        self.livros.buscar_por_autor(autor)
    }

    pub fn cadastrar_usuario(&mut self, nome: &str, id: &str) -> Result<(), String> {
        self.usuarios.adicionar_usuario(nome, id)
    }

    pub fn registrar_emprestimo(&mut self, usuario_id: &str, livro_codigo: &str) -> Result<(), String> {
        self.emprestimos.registrar_emprestimo(usuario_id, livro_codigo)
    }

    pub fn devolver_emprestimo(&mut self, usuario_id: &str, livro_codigo: &str) -> Result<(), String> {
        self.emprestimos.devolver_emprestimo(usuario_id, livro_codigo)
    }
}
