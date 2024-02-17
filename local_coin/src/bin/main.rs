use anyhow::Result;
use local_coin::substrate_runner::submit_transactions;

#[tokio::main]

async fn main() -> Result<()> {
    submit_transactions("local_coin/transactions.csv".to_string()).await;
    Ok(())
}