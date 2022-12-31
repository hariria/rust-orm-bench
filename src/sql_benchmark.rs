use crate::prisma::users;
use crate::PrismaClient;
use prisma_client_rust::{raw, PrismaValue};
use std::time::Instant;

pub async fn create_users_serially(client: &PrismaClient, guids: &Vec<String>) {
    for guid in guids {
        client
            ._execute_raw(raw!(
                "INSERT INTO users (id, display_name) VALUES ({}, {})",
                PrismaValue::String(guid.to_string()),
                PrismaValue::String("".to_string())
            ))
            .exec()
            .await
            .unwrap();
    }
}

pub async fn find_users_serially(client: &PrismaClient, guids: &Vec<String>) {
    for guid in guids {
        let _data: Vec<users::Data> = client
            ._query_raw(raw!(
                "SELECT * FROM users WHERE id = {}",
                PrismaValue::String(guid.to_string())
            ))
            .exec()
            .await
            .unwrap();
    }
}

pub async fn delete_users_serially(client: &PrismaClient, guids: &Vec<String>) {
    for guid in guids {
        client
            ._execute_raw(raw!(
                "DELETE FROM users WHERE id = {} AND display_name = {}",
                PrismaValue::String(guid.to_string()),
                PrismaValue::String("".to_string())
            ))
            .exec()
            .await
            .unwrap();
    }
}

pub async fn run_prisma_raw_sql_benchmark(guids: &Vec<String>) {
    let client: PrismaClient = PrismaClient::_builder().build().await.unwrap();
    println!("\n\nRunning prisma raw sql benchmarks...");
    let mut now = Instant::now();
    create_users_serially(&client, guids).await;
    println!("Create users elapsed {:.2?}", now.elapsed());
    now = Instant::now();
    find_users_serially(&client, guids).await;
    println!("Find users elapsed {:.2?}", now.elapsed());
    now = Instant::now();
    delete_users_serially(&client, guids).await;
    println!("Delete users elapsed {:.2?}", now.elapsed());
}
