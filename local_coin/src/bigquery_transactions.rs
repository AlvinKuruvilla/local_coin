use anyhow::Result;
use csv::Reader;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::errors::AddressPositionError;

#[derive(Debug, Deserialize, Serialize)]
/// Represents a record from a BigQuery CSV file.
///
/// This struct models the data structure for a record in a CSV file, primarily
/// focusing on an address and its associated value. The structure utilizes
/// serde's alias attribute to accommodate CSVs with slightly different column names.
///
/// # Fields
///
/// - `addresses`: The source address from the CSV. Can be found under the "source_address" column.
/// - `value`: The associated value for the given address. Can be located under either
///   the "delta" or "satoshi" columns in the CSV.
pub struct BigQueryCSVRecord {
    inputs_addresses: String,
    outputs_addresses: String,
    outputs_value: f64,
}
impl BigQueryCSVRecord {
    pub fn dest(&self) -> String {
        self.outputs_addresses.clone()
    }
    pub fn from(&self) -> String {
        self.inputs_addresses.clone()
    }
}
pub enum AddressType {
    Input,
    Output,
}
/// Given a filename as input return the value
/// column as a `Vec<i64>`
pub fn make_value_vector(filename: &str) -> Vec<f64> {
    let mut rdr = csv::Reader::from_path(filename).unwrap();
    let records: Vec<BigQueryCSVRecord> = rdr
        .deserialize()
        .map(|result| result.expect("Error parsing CSV record"))
        .collect();
    records
        .par_iter()
        .map(|record| record.outputs_value)
        .collect()
}
/// Retrieve the address and value columns in the dataframe as vectors
pub fn addresses_and_values_as_vectors(file_name: &str) -> (Vec<String>, Vec<String>, Vec<f64>) {
    let input_address_vec = make_address_vector(file_name, AddressType::Input);
    let output_address_vec = make_address_vector(file_name, AddressType::Output);
    let value_vec = make_value_vector(file_name);
    (input_address_vec, output_address_vec, value_vec)
}
/// Given a filename, a public address, and optional unique in that file, find its position within the address vector
pub fn get_address_position(
    filename: &str,
    public_address: String,
    value: Option<f64>,
    address_type: AddressType,
) -> Result<usize, AddressPositionError> {
    let address_vec = make_address_vector(filename, address_type); // This function should properly handle errors and possibly return Result

    if let Some(val) = value {
        let values = make_value_vector(filename); // This function should properly handle errors and possibly return Result
        find_matching_indices(&address_vec, &public_address, &values, &val)
            .map_err(|_| AddressPositionError::NoMatchingIndexForValue(public_address, val))
    } else {
        address_vec
            .par_iter()
            .position_first(|r| r == &public_address)
            .ok_or_else(|| AddressPositionError::NoMatchingAddress(public_address))
    }
}
/// Given a filename as input return the address column as a `Vec<String>`
// NOTE 1: The current implementation forces the returned Vec to be a
//         `Vec<String>`. If you need the value column call the `make_value_vector`
//          function
// NOTE 2: We cannot parallelize this function because doing par_bridge when
//         deserializing messes with element order guarantees vector gives
// TODO: Cache the vector?
pub fn make_address_vector(file_name: &str, address_type: AddressType) -> Vec<String> {
    let mut rdr = csv::Reader::from_path(file_name).unwrap();
    match address_type {
        AddressType::Input => {
            let records: Vec<String> = rdr
                .deserialize::<BigQueryCSVRecord>()
                .filter_map(|result| match result {
                    Ok(record) => Some(record.inputs_addresses.clone()),
                    Err(e) => {
                        println!("{:?}", e);
                        unreachable!()
                    }
                })
                .collect();
            records
        }
        AddressType::Output => {
            let records: Vec<String> = rdr
                .deserialize::<BigQueryCSVRecord>()
                .filter_map(|result| match result {
                    Ok(record) => Some(record.outputs_addresses.clone()),
                    Err(e) => {
                        println!("{:?}", e);
                        unreachable!()
                    }
                })
                .collect();
            records
        }
    }
}
fn find_matching_indices<T: PartialEq + ToString + Sync, U: PartialEq + ToString + Sync>(
    first_vector: &[T],
    val1: &T,
    second_vector: &[U],
    val2: &U,
) -> Result<usize, AddressPositionError> {
    assert_eq!(first_vector.len(), second_vector.len());
    first_vector
        .par_iter()
        .zip(second_vector.par_iter())
        .position_first(|(x, y)| x == val1 && y == val2)
        .ok_or_else(|| AddressPositionError::NoMatchingIndices {
            address: val1.to_string(),
            value: val2.to_string(),
        })
}
pub fn parse_as_records(file_path: String) -> Result<Vec<BigQueryCSVRecord>> {
    let mut rdr = Reader::from_path(file_path)?;
    let mut records = Vec::new();

    for result in rdr.deserialize() {
        let record: BigQueryCSVRecord = result?;
        records.push(record);
    }

    Ok(records)

}