use super::schema::users;
use super::schema::addresses;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name:  &'a str,
    pub email: &'a str
}

#[derive(Queryable)]
pub struct Address {
    pub id: i32,
    pub street_number: String,
    pub street_name: String,
    pub user_id: i32
}

#[derive(Insertable)]
#[table_name="addresses"]
pub struct NewAddress<'a> {
    pub street_number: &'a str,
    pub street_name:   &'a str,
    pub user_id:       &'a i32
}
