extern crate diesel_query;
extern crate diesel;
extern crate clap;

use self::diesel_query::*;
use self::models::{NewUser};

use clap::{Arg, App};

fn main() {
    let matches = App::new("create_user")
                      .arg(Arg::with_name("name")
                           .required(true)
                           .takes_value(true))
                      .arg(Arg::with_name("email")
                           .required(true)
                           .takes_value(true))
                      .get_matches();

    let connection = establish_connection();

    let user = create_user(&connection, NewUser {
        name:  matches.value_of("name").unwrap(),
        email: matches.value_of("email").unwrap(),
    });

    println!("Created user: {}", user.id);
}
