extern crate diesel_postgres;
extern crate diesel;

use self::diesel_postgres::*;
use self::diesel_postgres::models::{User};
use self::diesel::prelude::*;

fn main() {
    use diesel_postgres::schema::users::dsl::*;

    let connection = establish_connection();

    let results = users
        .order(id.desc())
        .limit(10)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());

    for result in results {
        println!("{}, {}, {}", result.id, result.name, result.email);
    }
}
