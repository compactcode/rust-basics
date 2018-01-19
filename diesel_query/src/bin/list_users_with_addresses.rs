extern crate diesel_query;
extern crate diesel;

use self::diesel_query::*;
use self::diesel_query::models::{User, Address};
use self::diesel::prelude::*;

fn main() {
    process().unwrap();
}

// 1. SELECT * FROM users
// 2. SELECT * FROM addresses WHERE user_id IN (users)
fn process() -> Result<(), diesel::result::Error> {
    use schema::users;

    let connection = establish_connection();

    let users = users::table
        .order(users::id.desc())
        .limit(10)
        .load::<User>(&connection)?;

    let addresses = Address::belonging_to(&users)
        .load::<Address>(&connection)?;

    let results = users.into_iter().zip(addresses).collect::<Vec<_>>();

    for (user, address) in results {
        println!("User: {}, {} Address: {}, {}", user.id, user.name, address.street_number, address.street_name);
    }

    Ok(())
}
