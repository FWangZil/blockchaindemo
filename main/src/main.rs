use core::blockchain;
use std::thread;
use std::time::Duration;

fn main() {
    let mut bc = blockchain::BlockChain::new_blockchain();

    println!("Start mining...");

    thread::sleep(Duration::from_secs(3));
    bc.add_block(String::from("a -> b: 5 btc"));
    thread::sleep(Duration::from_secs(3));
    bc.add_block("c -> d: 2 btc".to_string());

    for b in bc.blocks {
        println!("+++++++");
        println!("{:#?}", b);
        println!("+++++++");
    }
}