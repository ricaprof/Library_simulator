use serde::{Serialize, Deserialize};
use serde_json::{Value};
use std::fs;
use uuid::Uuid;

use std::result::Result; // Adicionar isso no início do arquivo



#[derive(Serialize, Deserialize, Debug)]
pub struct Livro {
    pub titulo: String,
    pub autor: String,
    pub ano: u32,
    pub codigo: String,
}

pub struct Livros {
    pub lista_livros: Vec<Livro>,
}

impl Livros {
    pub fn new() -> Self {
        let lista_livros = match fs::read_to_string("dados/dados.json") {
            Ok(data) => serde_json::from_str(&data).unwrap_or(Vec::new()),
            Err(_) => Vec::new(),
        };

        Livros { lista_livros }
    }

    pub fn adicionar_livro(&mut self, titulo: &str, autor: &str, ano: u32, codigo: &str) -> Result<(), String> {
        if self.lista_livros.iter().any(|livro| livro.codigo == codigo) {
            return Err("Código de livro duplicado".to_string());
        }

        let livro = Livro {
            titulo: titulo.to_string(),
            autor: autor.to_string(),
            ano,
            codigo: codigo.to_string(),
        };

        self.lista_livros.push(livro);
        self.salvar_dados();
        Ok(())
    }

    pub fn listar_livros(&self) {
        for livro in &self.lista_livros {
            println!("{:?}", livro);
        }
    }

    pub fn buscar_por_titulo(&self, titulo: &str) -> Result<(), String> {
        for livro in &self.lista_livros {
            if livro.titulo == titulo {
                println!("{:?}", livro);
                return Ok(());
            }
        }
        Err("Livro não encontrado".to_string())
    }

    pub fn buscar_por_autor(&self, autor: &str) -> Result<(), String> {
        for livro in &self.lista_livros {
            if livro.autor == autor {
                println!("{:?}", livro);
                return Ok(());
            }
        }
        Err("Autor não encontrado".to_string())
    }

    fn salvar_dados(&self) {
        let dados_json = serde_json::to_string(&self.lista_livros).unwrap();
        fs::write("dados/dados.json", dados_json).unwrap();
    }
}

