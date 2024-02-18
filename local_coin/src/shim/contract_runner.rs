use crate::errors::CommandError;
use anyhow::Result;
use std::io::Write;
use std::{
    io,
    process::{Command, Output},
};
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
/// Change the current directory to the specified one, execute the command with cargo, and revert back to the original directory.
/// __NOTE__: No salt is provided when instantiating the contract
/// # Arguments
///
/// * `dir` - The directory in which to execute the command.
///
/// # Return
/// The command output or an error if the command was unable to be executed.

pub fn instantiate_contract(dir: &str) -> Result<Output, CommandError> {
    let output: Output = Command::new("cargo")
        .arg("contract")
        .arg("instantiate")
        .args(["--constructor", "new", "--suri", "//Alice"])
        .current_dir(dir)
        .output()?;
    if !output.status.success() {
        eprintln!(
            "Command exited with non-zero status: {:?} and error: {:?}",
            output.status.code(),
            String::from_utf8(output.stderr.clone())
        );
    }
    Ok(output)
}
pub fn call_contract_function(
    dir: &str,
    contract_id: String,
    message: String,
) -> Result<Output, CommandError> {
    let mut args = vec![
        "contract",
        "call",
        "--contract",
        &contract_id,
        "--message",
        &message,
        "--suri",
        "//Alice",
    ];

    // Add "--dry-run" argument for "is_user_opted_in" message
    if message == "is_user_opted_in" {
        args.push("--dry-run");
    }

    let output = Command::new("cargo").args(args).current_dir(dir).output()?;

    if !output.status.success() {
        eprintln!(
            "Command exited with non-zero status: {:?} and error: {:?}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(output)
}

pub fn retrieve_contact_string(
    output: Result<Output, CommandError>,
) -> Result<String, CommandError> {
    match output {
        Ok(success_output) => {
            // stdout is assumed to be of type Vec<u8>.
            let stdout = String::from_utf8_lossy(&success_output.stdout);
            let membership_string = match stdout
                .to_string()
                .lines()
                .filter(|line| line.contains("contract"))
                .next()
            {
                Some(line) => Ok(line.to_string().trim().replace("contract: ", "")), // Return the line if "contract" is found.
                None => Ok("".to_owned()),
            };
            membership_string
        }
        Err(err) => {
            // Write the error to stderr.
            // Adjust this part according to the actual definition of CommandError.
            let _ = writeln!(io::stderr(), "Command Error: {:?}", err.to_string());
            Err(err)
        }
    }
}
