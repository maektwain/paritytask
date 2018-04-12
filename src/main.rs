
#[macro_use]

extern crate clap;
extern crate web3;

use web3::futures::Future;

use clap::App;
use web3::types::{Address, U256};

fn main() {

     let yaml = load_yaml!("cli.yml");

     let (_eloop, transport) = web3::transports::Http::new("http://localhost:8545").unwrap();
     let web3 = web3::Web3::new(transport);
     let matches = App::from_yaml(yaml).get_matches();
      // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
       // required we could have used an 'if let' to conditionally get the value)

        let address = matches.value_of("address").unwrap();
        let _format: Address = address.parse().unwrap();

        println!("Address balance is : {} ", _format );

        // 1. Send a transaction



        //2. Balance Query

        if matches.value_of("INPUT").unwrap() == "balance"{
            // This comes as vectors

            // for i in address {
            //     let balance = web3.eth().balance(i, None).wait().unwrap();
            //     println!("Address balance is : {:?} ", balance );
            // }

        }



        //3. Query Contract

        if matches.value_of("INPUT").unwrap() == "contract"{
            let accounts = web3.eth().accounts().wait().unwrap();
            // This comes as vectors

            for i in accounts {
                let balance = web3.eth().balance(i, None).wait().unwrap();
                println!("Address balance is : {:?} ", balance );
            }

        }

       // Vary the output based on how many times the user used the "verbose" flag
       // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
       match matches.occurrences_of("v") {
           0 => println!("No verbose info"),
           1 => println!("Some verbose info"),
           2 => println!("Tons of verbose info"),
           3 | _ => println!("Don't be crazy"),
       }

       // You can handle information about subcommands by requesting their matches by name
       // (as below), requesting just the name used, or both at the same time
       if let Some(matches) = matches.subcommand_matches("test") {
           if matches.is_present("debug") {
               println!("Printing debug info...");
           } else {
               println!("Printing normally...");
           }
       }
}
