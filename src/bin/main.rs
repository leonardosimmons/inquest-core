#![allow(unused)]
use std::rc::Rc;

use rusqlite;
use inquest::database;
use inquest::database::{DatabaseCreateController, Response};

fn main() {
    let person_table = vec![
        database::TableRow{ key: String::from("id"), attr: String::from("INTEGER PRIMARY KEY")},
        database::TableRow{ key: String::from("name"), attr: String::from("TEXT NOT NULL")},
        database::TableRow{ key: String::from("data"), attr: String::from("BLOB")}
    ];

    let connection = Rc::new(rusqlite::Connection::open_in_memory().unwrap());

    database::Controller::new(Rc::clone(&connection))
        .create()
        .table("person", person_table)
        .execute();
}
