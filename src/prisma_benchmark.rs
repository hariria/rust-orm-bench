use crate::prisma::users;
use crate::PrismaClient;
use std::time::Instant;

pub async fn create_users_serially(client: &PrismaClient, guids: &Vec<String>) {
    for guid in guids {
        client
            .users()
            .create(guid.to_string(), "".to_string(), vec![])
            .exec()
            .await
            .unwrap();
    }
}

pub async fn find_users_serially(client: &PrismaClient, guids: &Vec<String>) {
    for guid in guids {
        client
            .users()
            .find_unique(users::id::equals(guid.to_string()))
            .exec()
            .await
            .unwrap();
    }
}

pub async fn delete_users_serially(client: &PrismaClient, guids: &Vec<String>) {
    for guid in guids {
        client
            .users()
            .delete(users::id::equals(guid.to_string()))
            .exec()
            .await
            .unwrap();
    }
}

pub async fn run_prisma_benchmark(client: &PrismaClient, guids: &Vec<String>) {
    println!("\n\nRunning prisma benchmarks...");
    let mut now = Instant::now();
    create_users_serially(client, guids).await;
    println!("Create users elapsed {:.2?}", now.elapsed());
    now = Instant::now();
    find_users_serially(client, guids).await;
    println!("Find users elapsed {:.2?}", now.elapsed());
    now = Instant::now();
    delete_users_serially(client, guids).await;
    println!("Delete users elapsed {:.2?}", now.elapsed());
}
