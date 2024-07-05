// tests/database_tests.rs
use rusqlite::Connection;
use crate::modulos::database::Database;

#[test]
fn test_cria_db() {
    let db = Database::new(":memory:").unwrap();
    db.cria_db().unwrap();

    let conn = Connection::open_in_memory().unwrap();
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='produtos';").unwrap();
    let mut rows = stmt.query([]).unwrap();

    assert!(rows.next().unwrap().is_some());
}

#[test]
fn test_add_produto() {
    let db = Database::new(":memory:").unwrap();
    db.cria_db().unwrap();
    db.add_produto("Teste", "MarcaTeste", 1.0, 2.0).unwrap();

    let conn = Connection::open_in_memory().unwrap();
    let mut stmt = conn.prepare("SELECT Produto FROM produtos WHERE Produto = 'Teste';").unwrap();
    let mut rows = stmt.query([]).unwrap();

    assert!(rows.next().unwrap().is_some());
}

#[test]
fn test_show_produtos() {
    let db = Database::new(":memory:").unwrap();
    db.cria_db().unwrap();
    db.add_produto("Teste", "MarcaTeste", 1.0, 2.0).unwrap();

    let produtos = db.show_produtos().unwrap();
    assert_eq!(produtos.len(), 1);
    assert_eq!(produtos[0].1, "Teste");
}

#[test]
fn test_delete_produto() {
    let db = Database::new(":memory:").unwrap();
    db.cria_db().unwrap();
    db.add_produto("Teste", "MarcaTeste", 1.0, 2.0).unwrap();

    let produtos = db.show_produtos().unwrap();
    let id = produtos[0].0;

    db.delete_produto(id).unwrap();
    let produtos = db.show_produtos().unwrap();
    assert!(produtos.is_empty());
}
