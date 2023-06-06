use lmdb::{self,Database,Environment,EnvironmentFlags, Transaction};
use serde::de::DeserializeOwned;
use serde::{Serialize,Deserialize};
use std::path::Path;
use serde_pickle as pickle;

#[derive(Debug)]
pub struct LMDBReader {
    env:Environment,
    db:Database,
}



#[derive(Debug, Serialize, Deserialize)]
struct LMDBDataset {
    creation_date: String,
    sample_names: Vec<String>,
    data:Vec<Vec<String>>
}
impl LMDBReader {
    
    pub fn new(path:&str) -> Result<Self,lmdb::Error>{
        
   let env = Environment::new()
   .set_flags(EnvironmentFlags::NO_READAHEAD | EnvironmentFlags::NO_SUB_DIR)
   .open(Path::new(path))?;   

   let db = env.open_db(None)?;

   Ok(Self { env, db })
 
    }
    pub fn read(&self, key: &[u8]) -> Result<Option<Vec<u8>>, lmdb::Error> {
        let txn = self.env.begin_ro_txn()?;

        

    
        let result = match txn.get(self.db, &key) {
            Ok(value) => Some(value.to_vec()),
            Err(lmdb::Error::NotFound) => None,
            Err(err) => return Err(err),
        };

        

        Ok(result)
    }
}
impl Drop for LMDBReader {
    fn drop(&mut self) {
        self.env.sync(true).expect("Failed to sync LMDB environment");
    }
}


use std::collections::HashMap;

fn parse_lmdb_dataset<'a>(
    strain_names: &[&'a str],
    target_strains: &[&'a str],
    data: &HashMap<&'a str, Vec<String>>,
) -> (Vec<&'a str>, Vec<Vec<String>>) {
    let posit: Vec<usize> = strain_names
        .iter()
        .enumerate()
        .skip(1)
        .filter_map(|(idx, &strain)| {
            if target_strains.contains(&strain) {
                Some(idx)
            } else {
                None
            }
        })
        .collect();

    let sample_vals: Vec<&'a str> = posit
        .iter()
        .map(|&i| strain_names[i])
        .collect();

    let parsed_data: Vec<Vec<String>> = data
        .values()
        .map(|line| posit.iter().map(|&i| line[i].clone()).collect())
        .collect();

    (sample_vals, parsed_data)
}


// utitilities may decide to store in pickle
pub fn unpickle_data<T: DeserializeOwned>(data: &[u8]) -> Result<T, pickle::Error>
where
    T: DeserializeOwned,
{
    serde_pickle::from_slice(data, Default::default())
}



 
#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn test_lmdb_reader() {
        let temp_dir = tempdir::TempDir::new("lmdb_test").expect("Failed to create temporary directory");
        let path = temp_dir.path().to_str().unwrap();


        let reader = LMDBReader::new(path).expect("Failed to initialize reader");

        let key = b"my_key";
        let value = b"my_value";

        let result = reader.read(key).expect("Failed to read value from LMDB");
        assert_eq!(result, Some(value.to_vec()));
    }

    #[test]
    fn test_lmdb_reader2() {

        use lmdb::WriteFlags;
        // Create a temporary directory for the LMDB file
        let temp_dir = tempdir::TempDir::new("lmdb_test").expect("Failed to create temporary directory");
        let _path = temp_dir.path().to_str().unwrap();

        // Initialize the LMDBReader
        let reader = LMDBReader::new("./data").expect("Failed to initialize reader");

        // Open a write transaction and write data to the LMDB file
        let mut txn = reader.env.begin_rw_txn().expect("Failed to begin write transaction");
        unsafe {
            let dbi = txn.open_db(None).expect("Failed to open database");

            let key = b"my_key";
            let value = b"my_value";

            txn.put(dbi, key, value, WriteFlags::empty())
                .expect("Failed to write data to LMDB");
        }
        txn.commit().expect("Failed to commit transaction");

        let result = reader.read(b"my_key").expect("Failed to read value from LMDB");

        assert_eq!(result, Some(b"my_value".to_vec()));


    }
}


