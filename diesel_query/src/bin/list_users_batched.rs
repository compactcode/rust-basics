extern crate diesel_query;
extern crate diesel;

use self::diesel_query::*;
use self::diesel_query::models::{User};
use self::diesel::prelude::*;

fn main() {
    process().unwrap();
}

fn process() -> Result<(), diesel::result::Error> {
    use diesel_query::schema::users::dsl::*;

    let connection = establish_connection();

    let batch_size = 5;

    let mut load  = true;
    let mut batch = 0;
    let mut start = 0;

    while load {
        batch = batch + 1;

        let batch_results = users
            .order(id.asc())
            .filter(id.gt(start))
            .limit(batch_size)
            .load::<User>(&connection)?;

        if let Some(last) = batch_results.last() {
            println!("Displaying batch {} ", batch);

            for result in &batch_results {
                println!("{}, {}, {}", result.id, result.name, result.email);
            }

            start = last.id;
            load = true;
        } else {
            load = false;
        }
    }

    Ok(())
}
