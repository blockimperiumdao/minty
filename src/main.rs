use std::error::Error;
//use std::io;
//use std::fs;
use std::process;
//use std::env;
//use log::info;

use std::process::Command;

use serde::Deserialize;
use serde::Serialize;

use clap::Parser;

const CARGO_PKG_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[clap(short = 'F', long)]
    csv_file: Option<String>,

    #[clap(short = 'A', long)]
    account_id: Option<String>,

    #[clap(short = 'T', long)]
    publish_target: Option<String>
}


#[derive(Debug, Serialize, Deserialize)]
struct NFTRecord 
{
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

struct PublishOptions 
{
    csv_file: String,
    account_id: String,
    publish_target: String,
}

fn minty( publish_options: PublishOptions ) -> Result<(), Box<dyn Error>> 
{

    let mut rdr = csv::ReaderBuilder::new()
                        .delimiter(b'|')
                        .from_path(&publish_options.csv_file)?;
    let headers = rdr.headers()?;
    println!("Headers {:?}", headers);

    let account_id = publish_options.account_id;
    let publish_target = publish_options.publish_target;


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

        //let test = r#"'{"creator_id":"afrorick.testnet","token_metadata":{"title":"AfroRick","media":"bafybeid7ztbmhjx3266jm6fyoaft7xvwup2ex2da2odjvfl4s4pvxvgjni","copies":1},"price":"1000000000000000000000000","royalty":{"afrorick.testnet":1000}}' --depositYocto 8540000000000000000000"#;

        let token_medatada = format!(r#"{{"creator_id": "{account_id}","token_metadata": {{"title":"{title}","media":"{media}", "copies": {copies}, "attributes": [                        {
            "trait_type": "background",
            "value": "spring_sunny_day"
        },] }}, "price": "{price}", "royalty": {{"{royalty_act}": {royalty_pct} }} }}"#);


        println!( "Sending metadata : {}", token_medatada );

        let output = Command::new("near")
        .args(&["call","--accountId",&account_id, &publish_target ,"nft_create_series", &token_medatada, "--depositYocto",  "8540000000000000000000"])
        .output()
        .expect("Failed to execute command"); 

        println!("Output: {}", String::from_utf8_lossy(&output.stdout));
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));        
    }

    Ok(())
}

fn main() 
{

    println!("Minty... VERSION={}", CARGO_PKG_VERSION.unwrap_or("NOT_FOUND"));

    let args = Cli::parse();

    //let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    

    let mut publish_options = PublishOptions { csv_file: "".to_string(), 
                                                account_id: "".to_string(), 
                                                publish_target: "".to_string() };
    publish_options.csv_file = args.csv_file.expect("You must specify a file name");
    publish_options.account_id = args.account_id.expect("You must specify the account you are publishing to.");
    publish_options.publish_target = args.publish_target.unwrap_or("paras-token-v2.testnet".to_string());

    println!("Loading file {}", publish_options.csv_file);
    println!("Using account {}", publish_options.account_id);
    println!("Publishing to {}", publish_options.publish_target);

    if let Err(err) = minty( publish_options ) 
    {
        println!("error running example: {}", err);
        process::exit(1);
    }
}