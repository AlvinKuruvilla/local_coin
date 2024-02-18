use anyhow::Result;
use local_coin::{
    config::contract_path,
    shim::contract_runner::{
        call_contract_function, instantiate_contract, retrieve_contact_string,
    },
};

fn main() -> Result<()> {
    let out = instantiate_contract(&contract_path());
    let s = retrieve_contact_string(out)?;
    let res = call_contract_function(&contract_path(), s, "is_user_opted_in".to_string()).unwrap();
    println!(
        "{:?} {:?}",
        res.status.code(),
        String::from_utf8_lossy(&res.stdout)
    );
    Ok(())
}
