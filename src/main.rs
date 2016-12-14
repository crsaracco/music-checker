extern crate rusqlite;
extern crate getopts;

use std::env;
use std::string::String;
use getopts::Options;

mod database;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    let brief = format!("{}\n\nIf no options are given, this program will print out all missing releases.", brief);
    print!("{}", opts.usage(&brief));
}

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
        // -h detected; print help/usage
        print_usage(&program, opts);
        return;
    }

    // Check -n; if it matches, make the database and quit.
    if matches.opt_str("n").is_some() {
        let filename = matches.opt_str("n").unwrap();
        let database = database::Database::new(filename.clone());

        match database.create_artists_table() {
            Ok(_) => {},
            Err(e) => {
                println!("error: {}", e);
                return;
            },
        }

        match database.create_releases_table() {
            Ok(_) => {},
            Err(e) => {
                println!("error: {}", e);
                return;
            },
        }

        println!("Created new artist database: {}", filename);
        return;
    }

    // Check -c and -s (only one allowed); if match, check the single artist against MusicBrainz and quit.
    if matches.opt_str("c").is_some() || matches.opt_str("s").is_some() {
        panic!("TODO: -c / -s");
    }

    panic!("TODO: This would print out all missing releases.")
}
