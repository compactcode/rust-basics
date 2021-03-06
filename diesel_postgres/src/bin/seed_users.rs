extern crate diesel_postgres;
extern crate diesel;

use self::diesel_postgres::*;
use self::diesel_postgres::models::{NewUser, NewAddress};

fn main() {
    let connection = establish_connection();

    for i in 0..10 {
        let user = create_user(&connection, NewUser {
            name:  &format!("seed-{}", i),
            email: &format!("seed-{}@seed.com", i)
        });

        create_address(&connection, NewAddress {
            street_number: &format!("seed-street-number-{}", i),
            street_name:   &format!("seed-street-name-{}", i),
            user_id:       &user.id
        });
    }
}
