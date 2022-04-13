#![allow(unused)]
use clap::Parser;
use configparser::ini::Ini;
use std::error::Error;
use urlencoding::encode;

#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    config: std::path::PathBuf,
    message: String

}

use curl::easy::Easy;

fn main() -> Result<(), Box<dyn Error>> {
    let mut easy = Easy::new();
    let args = Cli::parse();
    let mut config = Ini::new();
    let map = config.load(&args.config)?;
    let  mut basic: String = config.get("DEFAULT", "basic").unwrap();
    let api_key = config.get("DEFAULT", "api_key").unwrap();
    let recipient = config.get("DEFAULT", "recipient").unwrap();
    basic.push_str(&api_key);
    basic.push_str("/sendMessage?chat_id=");
    basic.push_str(&recipient);
    basic.push_str("&text=");
    basic.push_str(&encode(&args.message));
    easy.url(&basic).unwrap();
    easy.perform().unwrap();
  Ok(())
}

