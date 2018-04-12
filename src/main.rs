
#[macro_use]

extern crate clap;
extern crate web3;

use web3::futures::Future;

use clap::App;
use web3::types::{Address, U256};

use web3::contract::{Contract, Options};

fn main() {

     let yaml = load_yaml!("cli.yml");

     let my_account: Address = "0083CD1DAA0677C99908a9180B9E171F8Ed150B4".parse().unwrap();

     let (_eloop, transport) = web3::transports::Http::new("https://ropsten.infura.io/").unwrap();
     let web3 = web3::Web3::new(transport);
     let matches = App::from_yaml(yaml).get_matches();
     let address = matches.value_of("address").unwrap();
     let _format: Address = address.parse().unwrap();





        //2. Balance Query
        if matches.value_of("INPUT").unwrap() == "balance"{
              let balance = web3.eth().balance(_format, None).wait().unwrap();
                    println!("Address balance is : {} ", balance);
        }



        //3. Query Contract
        if matches.value_of("INPUT").unwrap() == "contract"{
                println!("Address supplied is : {} ", _format);
                let contract = Contract::from_json(web3.eth(), _format, include_bytes!("abi.json")).unwrap();
                let result = contract.query("name", (my_account,), None, Options::default(), None);
                let result_of: String = result.wait().unwrap();
                println!("Smart Contract Address Name: {} ", result_of);
                let _balance_of = contract.query("balanceOf", (my_account,), None, Options::default(), None);
                let balance_of: U256 = _balance_of.wait().unwrap();
                println!("Smart Contract Balance : {} ", balance_of);

        }
       match matches.occurrences_of("v") {
           0 => println!("No verbose info"),
           1 => println!("Some verbose info"),
           2 => println!("Tons of verbose info"),
           3 | _ => println!("Don't be crazy"),
       }
       if let Some(matches) = matches.subcommand_matches("test") {
           if matches.is_present("debug") {
               println!("Printing debug info...");
           } else {
               println!("Printing normally...");
           }
       }
}
