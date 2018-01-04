extern crate diesel_query;
extern crate diesel;

use self::diesel_query::*;
use self::diesel_query::models::*;

fn main() {
    let connection = establish_connection();

    let user = create_user(&connection, NewUser {
        name:  "Shanon",
        email: "shanonmcquay@gmail.com"
    });

    println!("Created user: {}", user.id);
}
