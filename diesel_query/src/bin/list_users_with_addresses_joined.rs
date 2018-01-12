extern crate diesel_query;
extern crate diesel;

use self::diesel_query::*;
use self::diesel_query::models::{User, Address};
use self::diesel::prelude::*;

fn main() {
    process().unwrap();
}

// SELECT * FROM users INNER JOIN addresses ON users.id = addresses.user_id
fn process() -> Result<(), diesel::result::Error> {
    use schema::users;
    use schema::addresses;

    let connection = establish_connection();

    let results = users::table.inner_join(addresses::table).load::<(User, Address)>(&connection)?;

    for (user, address) in results {
        println!("User: {}, {} Address: {}, {}", user.id, user.name, address.street_number, address.street_name);
    }

    Ok(())
}
