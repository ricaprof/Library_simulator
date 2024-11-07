use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Usuario {
    pub nome: String,
    pub id: String,
}

pub struct Usuarios {
    pub lista_usuarios: Vec<Usuario>,
}

impl Usuarios {
    pub fn new() -> Self {
        let lista_usuarios = match fs::read_to_string("dados/dados.json") {
            Ok(data) => serde_json::from_str(&data).unwrap_or(Vec::new()),
            Err(_) => Vec::new(),
        };

        Usuarios { lista_usuarios }
    }

    pub fn adicionar_usuario(&mut self, nome: &str, id: &str) -> Result<(), String> {
        if self.lista_usuarios.iter().any(|usuario| usuario.id == id) {
            return Err("Usuário já existe".to_string());
        }

        let usuario = Usuario {
            nome: nome.to_string(),
            id: id.to_string(),
        };

        self.lista_usuarios.push(usuario);
        self.salvar_dados();
        Ok(())
    }

    fn salvar_dados(&self) {
        let dados_json = serde_json::to_string(&self.lista_usuarios).unwrap();
        fs::write("dados/dados.json", dados_json).unwrap();
    }
}
