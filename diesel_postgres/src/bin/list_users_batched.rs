extern crate diesel_postgres;
extern crate diesel;

use self::diesel::prelude::*;

use self::diesel_postgres::{establish_connection};
use self::diesel_postgres::models::{User};

fn main() {
    process().unwrap();
}

fn process() -> Result<(), diesel::result::Error> {
    for batch in BatchLoad::new() {
        println!("Displaying batch {} ", batch.number);

        for user in batch.data {
            println!("{}, {}, {}", user.id, user.name, user.email);
        }
    }

    Ok(())
}

struct BatchLoad {
    batch_size: i64,
    start_id: i32,
    current_batch: u32,
    connection: PgConnection
}

struct Batch<T> {
    number: u32,
    data: Vec<T>
}

impl BatchLoad {
    pub fn new() -> Self {
        BatchLoad {
            batch_size: 5,
            start_id: 0,
            current_batch: 0,
            connection: establish_connection()
        }
    }
}

// I'm interested in figuiring out if this can be made generic so it can be used across any table.
impl Iterator for BatchLoad {
    type Item = Batch<User>;

    fn next(&mut self) -> Option<Self::Item> {
        use diesel_postgres::schema::users::dsl::*;

        self.current_batch = self.current_batch + 1;

        let data = users
            .order(id.asc())
            .filter(id.gt(self.start_id))
            .limit(self.batch_size)
            .load::<User>(&self.connection)
            .expect(
                &format!("Failed to load batch: {}", &self.current_batch)
            );

        if let &Some(last_id) = &data.last().map(|last| last.id) {
            self.start_id = last_id;

            Some(
                Batch {
                    number: self.current_batch,
                    data
                }
            )
        } else {
            None
        }
    }
}
