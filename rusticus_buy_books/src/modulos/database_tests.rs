#[cfg(test)]
mod tests {
    use super::database::Database;
    #[test]
    fn test_cria_db() {
        let db = Database::new("test.db").expect("Erro ao criar a conexão com o banco de dados");
        assert!(db.cria_db().is_ok());
    }

    #[test]
    fn test_add_produto() {
        let db = Database::new("test.db").expect("Erro ao criar a conexão com o banco de dados");
        db.cria_db().expect("Erro ao criar banco de dados");
        assert!(db.add_produto("Produto Teste", "Marca Teste", 1.0, 10.0).is_ok());
    }

    #[test]
    fn test_show_produtos() {
        let db = Database::new("test.db").expect("Erro ao criar a conexão com o banco de dados");
        db.cria_db().expect("Erro ao criar banco de dados");
        db.add_produto("Produto Teste", "Marca Teste", 1.0, 10.0).expect("Erro ao adicionar produto");
        let produtos = db.show_produtos().expect("Erro ao mostrar produtos");
        assert_eq!(produtos.len(), 1);
    }

    #[test]
    fn test_delete_produto() {
        let db = Database::new("test.db").expect("Erro ao criar a conexão com o banco de dados");
        db.cria_db().expect("Erro ao criar banco de dados");
        db.add_produto("Produto Teste", "Marca Teste", 1.0, 10.0).expect("Erro ao adicionar produto");
        let produtos = db.show_produtos().expect("Erro ao mostrar produtos");
        let id = produtos[0].0;
        assert!(db.delete_produto(id).is_ok());
    }

    #[test]
    fn test_search_produtos() {
        let db = Database::new("test.db").expect("Erro ao criar a conexão com o banco de dados");
        db.cria_db().expect("Erro ao criar banco de dados");
        db.add_produto("Produto Teste", "Marca Teste", 1.0, 10.0).expect("Erro ao adicionar produto");
        let produtos = db.search_produtos("Produto Teste").expect("Erro ao pesquisar produtos");
        assert_eq!(produtos.len(), 1);
    }
}
