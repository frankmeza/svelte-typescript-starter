use crate::queries;
use postgres::Connection;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub id: i32,
    pub name: String,
}

pub fn fetch_people_list(conn: Connection) -> Vec<Person> {
    let mut people = Vec::new();
    let q = queries::get_name_id_people();

    for row in &conn.query(&q, &[]).expect("error on fetch_people_list") {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
        };

        people.push(person);
    }

    people
}

pub fn fetch_person_by_id(conn: Connection, id: &str) -> Person {
    let mut person = Person {
        id: 0,
        name: String::from(""),
    };

    let q = queries::get_name_id_person(id);

    for row in &conn.query(&q, &[]).expect("error on fetch_person_by_id") {
        let p = Person {
            id: row.get(0),
            name: row.get(1),
        };

        person = p;
    }

    person
}

pub fn create_person(conn: Connection, id: &str, name: &str) {
    let q = queries::create_person(id, name);
    &conn.execute(&q, &[]).expect("error on create_person");
}

pub fn update_person_by_id(conn: Connection, id: &str, name: &str) {
    let q = queries::update_person_by_id(id, name);
    &conn.execute(&q, &[]).expect("error on update_person_by_id");
}

pub fn delete_person_by_id(conn: Connection, id: &str) {
    let q = queries::delete_person_by_id(id);
    &conn.query(&q, &[]).expect("error on delete_person_by_id");
}
