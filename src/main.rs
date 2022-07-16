use clap::Parser;
use std::collections::HashMap;

mod cache;

#[derive(Parser)]
struct CLI {
    method: String,
    key: String,
    value: String,
}

fn main() {
    let args = CLI::parse();
    let method: &str = &*args.method;
    let key: &str = &*args.key;
    let value: &str = &*args.value;

    let mut store: HashMap<&str, &str> = HashMap::new();

    match method {
        "get" => {
            let value = cache::get(store, key);

            match value {
                "" => println!("key not found"),
                _ => println!("got value {}", value)
            }
            
        },
        "set" => {
            cache::set(&mut store, key, value);

            let value = cache::get(store, key);
            println!("inserted value {}", value)
        }
        _ => {
            println!("invalid method")
        }
    }
}
