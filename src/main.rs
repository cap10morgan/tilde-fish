use clojure_reader::edn;
use std::io::Read;
use tilde_fish::{fish_config, plugin_config};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "--config" {
        let config = plugin_config();
        println!("{}", config);
        return;
    }

    if args.len() > 1 && args[1] == "--gen-config" {
        // Read from stdin
        let mut input = String::new();
        std::io::stdin()
            .read_to_string(&mut input)
            .expect("Failed to read from stdin");

        // Parse the EDN
        let cfg = edn::read_string(&input).expect("Failed to parse EDN from stdin");

        let config = fish_config(cfg);
        println!("{}", config);
        return;
    }

    if args.len() < 3 {
        eprintln!(
            "Usage: {} [pattern] [path] or {} --config or {} --gen-config < input.edn",
            args[0], args[0], args[0]
        );
        std::process::exit(1);
    }
}
