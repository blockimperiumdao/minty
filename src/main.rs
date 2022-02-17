use std::error::Error;
use std::io;
use std::fs;
use std::process;
use std::env;

use std::process::Command;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct NFTRecord {
    title: String,
    description: String,
    media: String,
    media_hash: String,
    copies: u128,
    extra: String,
    price: u128,
    royalty_account: String,
    royalty_pct: u16,
}

fn minty( filename: &String, account_id: &String ) -> Result<(), Box<dyn Error>> {

    let mut rdr = csv::Reader::from_path(filename)?;
    let headers = rdr.headers()?;
    println!("Headers {:?}", headers);

    for result in rdr.deserialize() 
    {
        println!("Found record");

        let mut record: NFTRecord = result?;

        record.royalty_pct = record.royalty_pct * 1000;

        println!("{:?}", record);

        let test = r#"'{"creator_id":"afrorick.testnet","token_metadata":{"title":"AfroRick","media":"bafybeid7ztbmhjx3266jm6fyoaft7xvwup2ex2da2odjvfl4s4pvxvgjni","copies":1},"price":"1000000000000000000000000","royalty":{"afrorick.testnet":1000}}' --depositYocto 8540000000000000000000"#;


        let price = record.price as u32;
        let copies = record.copies as u32;
        let mut token_metadata = json::JsonValue::new_object();
        token_metadata["title"] = record.title.into();
        token_metadata["media"] = record.media.into();
        token_metadata["reference"] = record.media_hash.into();
        token_metadata["copies"] = copies.into();

        let mut data = json::JsonValue::new_object();
        data["creator_id"] = account_id.to_string().into();
        data["token_metadata"] = token_metadata.to_string().into();
        data["price"] = price.into();

        println!("{:?}", data.dump());


        // let output = if cfg!(target_os = "windows")
        // {
        //     Command::new("cmd")
        //                 .arg("near")
        //                 .arg("call")
        //                 .output()
        //                 .expect("Failed to execute command")
        // }
        // else 
        // {
        //     println!("Running the unix command...");
        //     // Command::new("/bin/sh")
        //     //             .arg("-c")
        //     //             .arg("near")
        //     //             .arg("call")
        //     //             .arg("--accountId")
        //     //             .arg(account_id)
        //     //             .arg("paras-token-v2.testnet")
        //     //             .arg("nft_create_series")
        //     //             .arg( data.dump() )
        //     //             .output()
        //     //             .expect("Failed to execute command")
        //     //Command::new("sh").arg("-C").arg("near call --accountId afrorick.testnet paras-token-v2.testnet nft_create_series").output().expect("Failed")
        //     Command::new("/bin/sh")
        //         .args(&["-c"])
        //         .spawn()

        // };

        // let output = Command::new("/bin/sh")
        //     .args(&["-c","near","call","--accountId afrorick.testnet", "paras-token-v2.testnet"])
        //     .output()
        //     .expect("Failed to execute command");

        // let output = Command::new("near")
        // .args(&["state","afrorick.testnet"])
        // .output()
        // .expect("Failed to execute command");        

        let output = Command::new("near")
        .args(&["call","--accountId",account_id,"paras-token-v2.testnet","nft_create_series", &test])
        .output()
        .expect("Failed to execute command"); 

        //println!("{:?}", output);
        //println!("Status: {}", output.);
        println!("Output: {}", String::from_utf8_lossy(&output.stdout));
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));


        //TODO: We need to deserialize the StringRecord somehow.

        // Notice that we need to provide a type hint for automatic
        // deserialization.
        // let record: NFTRecord = result?;
        // println!("{:?}", record);
        // let record = result?;
        // record.deserialize(NFTRecord);

        // StringRecord.deserialize(headers: Option<&'de StringRecord>)
    }

    // // deserialize each of the records in the CSV into its own NFTRecord struct
    // for result in rdr.deserialize() 
    // {

 
    // }
    Ok(())
}

fn main() {



    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    
    let csv_file = &args[1];
    let account_id = &args[2];
    println!("Loading file {}", csv_file);
    println!("Using account {}", account_id);

    if let Err(err) = minty( csv_file, account_id ) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}