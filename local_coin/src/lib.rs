use std::io::{stdin, stdout, Read, Write};

extern crate lazy_static;

pub mod bigquery_transactions;
pub mod config;
pub mod errors;
pub mod shim;
pub(crate) fn _pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
