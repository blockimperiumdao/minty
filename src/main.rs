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

        // ensure the royalty pct is in the format required by paras
        //
        record.royalty_pct = record.royalty_pct * 1000;

        //println!("{:?}", record);



        let price = record.price as u32;
        let copies = record.copies as u32;
        let title = record.title;
        let media = record.media;
        let royalty_act = record.royalty_account;
        let royalty_pct = record.royalty_pct;


        let test = r#"'{"creator_id":"afrorick.testnet","token_metadata":{"title":"AfroRick","media":"bafybeid7ztbmhjx3266jm6fyoaft7xvwup2ex2da2odjvfl4s4pvxvgjni","copies":1},"price":"1000000000000000000000000","royalty":{"afrorick.testnet":1000}}' --depositYocto 8540000000000000000000"#;

        let token_medatada = format!(r#"{{"creator_id": "{account_id}","token_metadata": {{"title":"{title}","media":"{media}", "copies": {copies} }}, "price": "{price}", "royalty": {{"{royalty_act}": {royalty_pct} }} }}"#);


        println!( "Sending metadata : {}", token_medatada );

        let output = Command::new("near")
        .args(&["call","--accountId",account_id,"paras-token-v2.testnet","nft_create_series", &token_medatada, "--depositYocto",  "8540000000000000000000"])
        .output()
        .expect("Failed to execute command"); 

        println!("Output: {}", String::from_utf8_lossy(&output.stdout));
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));        
    }

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