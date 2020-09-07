use std::env;
use std::path::PathBuf;
use std::process;

use heart::rocket;

fn main() {
    let usage = usage();
    let csv_path = env::args_os().nth(1).expect(&usage);

    match rocket(&PathBuf::from(csv_path)) {
        Ok(r) => r.launch(),
        Err(e) => {
            println!("{:?}", e);
            process::exit(1);
        }
    };
}

fn usage() -> String {
    let program = env::args_os().next().expect("could not get program name");
    format!("{:?} CSV_FILE", program)
}
