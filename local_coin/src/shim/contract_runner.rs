use crate::bigquery_transactions::parse_as_records;
use anyhow::Result;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;
use sysinfo::System;

pub fn is_substrate_node_running() -> bool {
    let s = System::new_all();
    if s.processes_by_name("substrate-contracts-node")
        .peekable()
        .peek()
        .is_some()
    {
        return true;
    }
    false
}

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod polkadot {}

pub async fn submit_transactions(file_path: String) -> Result<()> {
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let recs = parse_as_records(file_path).unwrap();
    // TODO: The end goal is to use outputs_addresses field for this, but kept getting
    //       Phrase(BadWordCount(1))
    //       Keypair::from_uri(&SecretUri::from_str(&rec.dest()).unwrap())
    //       .unwrap()
    //       .public_key()
    //       .into(),
    let dest = dev::bob().public_key().into();
    let balance_transfer_tx = polkadot::tx().balances().transfer_allow_death(dest, 10_000);
    // TODO: The end goal is the same as above except with input addresses
    //       &Keypair::from_uri(&SecretUri::from_str(&rec.from()).unwrap()).unwrap(),
    let from = dev::alice();
    println!("Here");
    let progress = api
        .tx()
        .sign_and_submit_then_watch_default(&balance_transfer_tx, &from)
        .await?;
    println!("Are we Here?");

    let events = progress.extrinsic_hash();
    println!("Not Here");
    // Find a Transfer event and print it.
    // let transfer_event = events.find_first::<polkadot::balances::events::Transfer>()?;
    // if let Some(event) = transfer_event {
    //     println!("Balance transfer success: {event:?}");
    // }

    Ok(())
}
