#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
struct User {
	pub id: i32,
    pub name: String,
    pub email: String
}

fn create_user(id: i32) -> User {
    User {
        id,
        name: String::from("Shanon"),
        email: String::from("shanon@test.com")
    }
}

fn main() {
    let users = (1..5).map(|id| create_user(id));

    let json = users.map(|user| serde_json::to_string(&user).unwrap()).collect::<Vec<String>>();

    println!("json = {:?}", json);
}
