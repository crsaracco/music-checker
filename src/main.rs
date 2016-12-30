extern crate getopts;

use std::env;
use std::string::String;
use getopts::Options;
use std::process::exit;

mod database; // TODO: "use database::Database"

fn main() {
    // Get raw arguments to pass to getopts
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // Make a new getopts::Options object
    let mut opts = Options::new();

    // List all possible arguments to be parsed
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("n", "new", "Create a new database with given name", "NAME");
    opts.optflag("c", "check", "Check all artists against the MusicBrainz database");
    opts.optflagopt("s", "check-single", "Check only a single artist against the MusicBrainz database. By default, this checks the least-recently-checked aritst.", "ARTIST");

    // Make getopts parse our arguments
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    // Check -h; if it matches, print help and quit.
    if matches.opt_present("h") {
        print_usage(&program, opts);
    }

    // Check -n; if it matches, make the database and quit.
    if matches.opt_present("n") {
        let filename = matches.opt_str("n").unwrap(); // unwrap is okay because opts.parse checks this for us (opts.optopt requires an argument)
        create_new_database(filename);
        return;
    }

    // Check -c and -s (only one allowed); if match, check the single artist against MusicBrainz and quit.
    let opt_c_present = matches.opt_present("c");
    let opt_s_present = matches.opt_present("s");
    if opt_c_present && opt_s_present {
        println!("ERROR: Only one of the (-s / -c) arguments is allowed!");
        exit(1);
    }
    else if opt_c_present {
        check_all_artists();
    }
    else if opt_s_present {
        match matches.opt_str("s") {
            Some(s) => {
                check_single_artist_given(s);
            },
            None => {
                check_single_artist_least_recent();
            }
        }
    }

    print_missing_releases();
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    let brief = format!("{}\n\nIf no options are given, this program will print out all missing releases.", brief);
    print!("{}", opts.usage(&brief));
    exit(1);
}

fn create_new_database(filename: String) {
    let database = database::Database::new(filename.clone());
    match database.create_artists_table() {
        Ok(_) => {},
        Err(e) => {
            println!("ERROR: {}", e);
            exit(1);
        },
    }
    match database.create_releases_table() {
        Ok(_) => {},
        Err(e) => {
            println!("ERROR: {}", e);
            exit(1);
        },
    }
    println!("Created new artist database: {}", filename);
}

fn check_all_artists() {
    unimplemented!();
}

fn check_single_artist_given(s: String) {
    unimplemented!();
}

fn check_single_artist_least_recent() {
    unimplemented!();
}

fn print_missing_releases() {
    unimplemented!();
}
