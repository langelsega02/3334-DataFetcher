use std::io::Write;
use std::thread;
use std::time::Duration;
use std::fs::OpenOptions;

// Struct definitions with price member to store prices
#[derive(Debug)]
struct Bitcoin {
    price: f64,
}

#[derive(Debug)]
struct Ethereum {
    price: f64,
}

#[derive(Debug)]
struct SP500 {
    price: f64,
}

// trait for the two main functions of the structs
trait Pricing {
    fn fetch_price(&mut self) -> Result<(), String>;
    fn save_to_file(&self);
}

// iimplementing Pricing trait for Bitcoin struct
impl Pricing for Bitcoin {
    fn fetch_price(&mut self) -> Result<(), String>{
        let bitcoin_api = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
        let response: serde_json::Value = ureq::get(bitcoin_api)
            .call()
            .map_err(|e| format!("Error when accessing: {}", e))?
            .into_json()
            .map_err(|e| format!("Error when reading: {}", e))?;
        if let Some(price) = response["bitcoin"]["usd"].as_f64() {
            self.price = price;
            println!("Bitcoin Price: ${}", self.price);
            Ok(())
        } else {
            Err("Error when fetching price".to_string())
        }
    }

    fn save_to_file(&self){ //function to create a txt file where prices are sent
        let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("bitcoin.txt")
        .unwrap();
    writeln!(file, "Bitcoin Price: ${}", self.price).unwrap();
    }
}

// impmlementing Pricing trait for Ethereum
impl Pricing for Ethereum {//function to fetch prices from the free api used
    fn fetch_price(&mut self) -> Result<(), String>{//function to fetch prices from the free api used
        let ethereum_api = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
        let response: serde_json::Value = ureq::get(ethereum_api)
            .call()
            .map_err(|e| format!("Error when accessing: {}", e))?
            .into_json()
            .map_err(|e| format!("Error when reading: {}", e))?;
        if let Some(price) = response["ethereum"]["usd"].as_f64() {
            self.price = price;
            println!("Ethereum Price: ${}", self.price);
            Ok(())
        } else {
            Err("Error when fetching price".to_string())
        }
    }

    fn save_to_file(&self){ //function to create a txt file where prices are sent
        let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("ethereum.txt")
        .unwrap();
    writeln!(file, "Ethereum Price: ${}", self.price).unwrap();
    }
}

// implementing Pricing trait for SP500 struct
impl Pricing for SP500 { //due to not finding a free api for SP500, I used the tether crypto as a replacement, but the implementation is the same
    fn fetch_price(&mut self) -> Result<(), String>{ //function to fetch prices from the free api used
        let tether_api = "https://api.coingecko.com/api/v3/simple/price?ids=tether&vs_currencies=usd";
        let response: serde_json::Value = ureq::get(tether_api)
            .call()
            .map_err(|e| format!("Error when accessing: {}", e))?
            .into_json()
            .map_err(|e| format!("Error when reading: {}", e))?;
        if let Some(price) = response["tether"]["usd"].as_f64() {
            self.price = price;
            println!("SP500 (which in truth is tether simulating to be SP500) Price: ${}", self.price);
            Ok(())
        } else {
            Err("Error when fetching price".to_string())
        }
    }

    fn save_to_file(&self){ //function to create a txt file where prices are sent
        let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("sp500.txt")
        .unwrap();
    writeln!(file, "S&P500 Price: ${}", self.price).unwrap();
    }
}

// main function
fn main() {
    //create 3 instances of each struct
    let mut bitcoin = Bitcoin { price: 0.0 };
    let mut ethereum = Ethereum { price: 0.0 };
    let mut sp500 = SP500 { price: 0.0 };

    //create a vector with the structs to iterate through during the loop
    let mut assets: Vec<&mut dyn Pricing> = vec![&mut bitcoin, &mut ethereum, &mut sp500];

    loop {
        for asset in &mut assets {
            if let Err(e) = asset.fetch_price() { //i have unreliable connection, 
            // and I was failing to fetch the prices a lot, so I found this should return an error when fetching from the api.
                eprintln!("Failed to fetch price: {}", e);
            }
            asset.save_to_file();
        }
        println!("Data saved. Waiting for the next cycle...");
        thread::sleep(Duration::from_secs(10)); //program sleeps for 10 seconds before trying to fetch prices again.
    }
}
