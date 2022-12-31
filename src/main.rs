mod constants;
mod diesel_benchmark;
mod prisma;
mod prisma_benchmark;
mod sql_benchmark;

use crate::diesel_benchmark::run_diesel_benchmark;
use crate::prisma_benchmark::run_prisma_benchmark;
use crate::sql_benchmark::run_prisma_raw_sql_benchmark;
use prisma::PrismaClient;
use uuid::Uuid;

fn generate_guids() -> Vec<String> {
    let mut result = vec![];
    let mut index = 0;
    while index < 10_000 {
        result.push(Uuid::new_v4().to_string());
        index += 1;
    }
    result
}

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    let guids = generate_guids();
    run_diesel_benchmark(&guids);
    run_prisma_benchmark(&guids).await;
    run_prisma_raw_sql_benchmark(&guids).await;
}
