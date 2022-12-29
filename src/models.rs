use super::schema::users;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct UserModel {
    pub id: String,
    pub display_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub display_name: &'a str,
}
