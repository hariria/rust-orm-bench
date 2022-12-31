use crate::prisma::users;
use crate::PrismaClient;
use std::sync::Arc;
use std::time::Instant;

pub async fn create_users_serially(client: Arc<PrismaClient>, guids: &Vec<String>) {
    for guid in guids {
        client
            .users()
            .create(guid.to_string(), "".to_string(), vec![])
            .exec()
            .await
            .unwrap();
    }
}

#[allow(dead_code)]
pub async fn create_users_concurrently(client: Arc<PrismaClient>, guids: &Vec<String>) {
    let mut tasks = Vec::with_capacity(guids.len());
    guids.iter().for_each(|guid| {
        let guid_clone = guid.clone();
        let client_clone = client.clone();
        tasks.push(tokio::spawn(async move {
            client_clone
                .users()
                .create(guid_clone.to_string(), "".to_string(), vec![])
                .exec()
                .await
                .unwrap();
        }));
    });
    for task in tasks {
        task.await.unwrap();
    }
}

#[allow(dead_code)]
pub async fn batch_create_users(client: Arc<PrismaClient>, guids: &Vec<String>) {
    let mut batch_vec = Vec::with_capacity(guids.len());
    guids.iter().for_each(|guid| {
        (&mut batch_vec).push(
            client
                .users()
                .create(guid.to_string(), "".to_string(), vec![]),
        );
    });
    client._batch(batch_vec).await.unwrap();
}

pub async fn find_users_serially(client: Arc<PrismaClient>, guids: &Vec<String>) {
    for guid in guids {
        client
            .users()
            .find_unique(users::id::equals(guid.to_string()))
            .exec()
            .await
            .unwrap();
    }
}

pub async fn delete_users_serially(client: Arc<PrismaClient>, guids: &Vec<String>) {
    for guid in guids {
        client
            .users()
            .delete(users::id::equals(guid.to_string()))
            .exec()
            .await
            .unwrap();
    }
}

pub async fn run_prisma_benchmark(guids: &Vec<String>) {
    let client = Arc::new(PrismaClient::_builder().build().await.unwrap());
    println!("\n\nRunning prisma benchmarks...");
    let mut now = Instant::now();
    create_users_serially(client.clone(), guids).await;
    println!("Create users elapsed {:.2?}", now.elapsed());
    now = Instant::now();
    find_users_serially(client.clone(), guids).await;
    println!("Find users elapsed {:.2?}", now.elapsed());
    now = Instant::now();
    delete_users_serially(client.clone(), guids).await;
    println!("Delete users elapsed {:.2?}", now.elapsed());
}
