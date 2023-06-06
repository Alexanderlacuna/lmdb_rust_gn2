pub mod parse;
use parse::LMDBReader;
use parse::unpickle_data;
use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct MyStruct {
    name:String,
    result: Vec<Option<f64>>
}

use std::collections::HashMap;
use std::vec;
#[derive(Debug, Serialize, Deserialize)]
struct MyStruct2 {
    data:HashMap<String,Vec<f64>>,
    creation_date: String,
    strain_names:Vec<String>
}


struct Meta{
    lmdb_target_path:String,
    primary_trait: Vec<f64>,
    primary_sample_names:Vec<String>
}


struct Parse {
    data:MyStruct2,
    primary_trait:Meta
    
}




fn main() {
    println!("Hello, world!");

    use std::path::Path;
    use std::time::Instant;
    let now = Instant::now();
    let w = Path::new("/tmp/Probesets/data.mdb").to_str().unwrap();
    println!("the value is {w}");
     let target_name = b"ProbeSetFreezeId_112_Hippocampus_Consortium_M430v2_(Jun06)_PDNN";      
    let results = match LMDBReader::new(w){
        Ok(data) => data.read(target_name).unwrap(),
        Err(some_error) => panic!("this is the error {some_error}")
    }.unwrap();
    println!("the time taken to read is {:?}",now.elapsed());
    let now = Instant::now();
   let x:MyStruct2= unpickle_data(&results).unwrap();
   println!("the time to unpickle this {:?}",now.elapsed());
       // pre prarse
       //optimized gn2 lmdb
       struct Meta{
        lmdb_target_path:String,
        primary_trait: Vec<f64>,
        primary_sample_names:Vec<String>
    }
    let target_trait_data = Meta {

        lmdb_target_path:String::from("/tmp_path"),
        primary_sample_names:x.strain_names.clone(),
        primary_trait:vec![12.,15.4,124.5,12.1,15.1,12.]


    };

    
    let now = Instant::now();
    let results = pre_parse(x, target_trait_data);

    println!("the time to parse the data this {:?}",now.elapsed());

       fn pre_parse(data:MyStruct2,target_trait:Meta)-> (String,Vec<f64>){
        let set_b:Vec<String> = target_trait.primary_sample_names.iter().cloned().collect();
        let target_indexes:Vec<usize> = data.strain_names    
        .iter()
        .enumerate()

        .filter_map(|(i, item_a)| set_b.contains(item_a).then(|| i))
        .collect();


   
   let selected_values: HashMap<String, Vec<Option<&f64>>> = data.data
        .iter()
        .map(|(key, values)| {

            



            
            let parsed_y_vals: Vec<Option<&f64>> = target_indexes
                .iter()
                .map(|&index|         
                    match (values.get(index),target_trait.primary_trait.get(index)){
                        (Some(val_y),Some(val_x)) => {
                          //parse_x_vals.append(*val_x);

                          

                          

                          Some(val_y)

                        }

                        _ => None,
                    }
                
                )
                .collect();
            //correlation here do them 
            if (parsed_y_vals.len() < 4 ){
            
            }            
            (key.clone(), parsed_y_vals)
        })
        .collect();


    //do correlation



    //do correlation
      


        

        (String::from("hello"),vec![1.,4.,5.])

       }




    
    }

    





    