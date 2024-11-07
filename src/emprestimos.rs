use serde::{Serialize, Deserialize};
use std::fs;

use std::result::Result; // Adicionar isso no início do arquivo

#[derive(Serialize, Deserialize, Debug)]
pub struct Emprestimo {
    pub usuario_id: String,
    pub livro_codigo: String,
    pub devolvido: bool,
}

pub struct Emprestimos {
    pub lista_emprestimos: Vec<Emprestimo>,
}

impl Emprestimos {
    pub fn new() -> Self {
        let lista_emprestimos = match fs::read_to_string("dados/dados.json") {
            Ok(data) => serde_json::from_str(&data).unwrap_or(Vec::new()),
            Err(_) => Vec::new(),
        };

        Emprestimos { lista_emprestimos }
    }

    pub fn registrar_emprestimo(&mut self, usuario_id: &str, livro_codigo: &str) -> Result<(), String> {
        if self.lista_emprestimos.iter().any(|emp| emp.livro_codigo == livro_codigo && !emp.devolvido) {
            return Err("Livro já emprestado".to_string());
        }

        let emprestimo = Emprestimo {
            usuario_id: usuario_id.to_string(),
            livro_codigo: livro_codigo.to_string(),
            devolvido: false,
        };

        self.lista_emprestimos.push(emprestimo);
        self.salvar_dados();
        Ok(())
    }

    pub fn devolver_emprestimo(&mut self, usuario_id: &str, livro_codigo: &str) -> Result<(), String> {
        match self.lista_emprestimos.iter_mut().find(|emprestimo| {
            emprestimo.usuario_id == usuario_id && emprestimo.livro_codigo == livro_codigo
        }) {
            Some(emprestimo) => {
                // Devolve o livro
                emprestimo.devoldido = false;
                Ok(()) // Retorna Ok indicando sucesso
            }
            None => Err("Empréstimo não encontrado ou já devolvido".to_string()), // Retorna erro caso não encontre
        }
    }
    

    fn salvar_dados(&self) {
        let dados_json = serde_json::to_string(&self.lista_emprestimos).unwrap();
        fs::write("dados/dados.json", dados_json).unwrap();
    }
}
