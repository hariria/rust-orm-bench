pub mod models;
pub mod schema;

use diesel::prelude::*;
pub mod constants;

use crate::constants::ABSOLUTE_FILE_PATH;
use crate::models::NewUser;
use crate::schema::users;

pub fn establish_connection() -> SqliteConnection {
    let database_url = ABSOLUTE_FILE_PATH.to_string();
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &mut SqliteConnection, id: &str, display_name: &str) -> usize {
    let new_user = NewUser { id, display_name };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user")
}

pub fn delete_user(conn: &mut SqliteConnection, param_id: &str) -> usize {
    use crate::schema::users::dsl::*;

    diesel::delete(users.filter(id.eq(param_id)))
        .execute(conn)
        .expect("Error deleting user")
}

// pub fn find_user(conn: &mut SqliteConnection, param_id: &str) {
//     use crate::schema::users::dsl::*;
//
//     users.filter(id.eq(param_id))
//         .limit(1)
//         .load::<users>(conn)
//         .expect("Error loading posts");
// }
