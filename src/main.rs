use libfra::{load_dictionary, should_be_flagged, pairs_from_string};

use std::path::Path;

use std::sync::mpsc::{Receiver, Sender, channel};
use std::collections::HashSet;


pub fn run_worker(
    rx: Receiver<String>,
    tx: Sender<(&str, &str)>
) {
    std::thread::spawn(move || {
        let dict = load_dictionary(Path::new("wordlist.txt"));

        loop {
            let message = rx.recv().unwrap();

            for (a, b) in should_be_flagged(&pairs_from_string(&message), &dict) {
                println!("Warning possible särskrivning {} {}", a, b);
            }
        }
    });
}


fn main() {
    let result = pairs_from_string("Edvard gillar att sär skriva");
}
