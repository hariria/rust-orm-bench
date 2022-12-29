use diesel::SqliteConnection;
use orm_bench::{create_user, delete_user, establish_connection};
use tokio::time::Instant;

pub fn create_users_serially(sqlite_conn: &mut SqliteConnection, guids: &Vec<String>) {
    for guid in guids {
        create_user(sqlite_conn, guid.as_str(), "");
    }
}

pub fn delete_users_serially(sqlite_conn: &mut SqliteConnection, guids: &Vec<String>) {
    for guid in guids {
        delete_user(sqlite_conn, guid.as_str());
    }
}

pub fn run_diesel_benchmark(guids: &Vec<String>) {
    let sqlite_conn = &mut establish_connection();
    println!("\n\nRunning diesel benchmarks...");
    let mut now = Instant::now();
    create_users_serially(sqlite_conn, guids);
    println!("Create users elapsed {:.2?}", now.elapsed());
    now = Instant::now();
    delete_users_serially(sqlite_conn, guids);
    println!("Delete users elapsed {:.2?}", now.elapsed());
}
