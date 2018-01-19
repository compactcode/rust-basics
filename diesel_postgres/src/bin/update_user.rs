extern crate diesel_postgres;
extern crate diesel;
#[macro_use]
extern crate clap;

use self::diesel::prelude::*;
use self::diesel_postgres::*;
use self::models::User;

use clap::{Arg, App};

fn main() {
    use diesel_postgres::schema::users::dsl::{users, email};

    let matches = App::new("update_user")
                      .arg(Arg::with_name("id")
                           .required(true)
                           .takes_value(true))
                      .arg(Arg::with_name("email")
                           .required(true)
                           .takes_value(true))
                      .get_matches();

    let id        = value_t!(matches, "id", i32).unwrap_or_else(|e| e.exit());
    let new_email = value_t!(matches, "email", String).unwrap_or_else(|e| e.exit());

    let connection = establish_connection();

    let user = diesel::update(users.find(id))
        .set(email.eq(new_email))
        .get_result::<User>(&connection)
        .expect(&format!("Unable to find user {}", id));

    println!("Updated user email to {}", user.email);
}
