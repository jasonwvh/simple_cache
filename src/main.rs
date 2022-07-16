use clap::Parser;
mod get;
mod set;

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

    match method {
        "get" => {
            get::get(key);
        },
        "set" => {
            set::set(key, value);
        }
        _ => {
            println!("invalid method")
        }
    }
}
