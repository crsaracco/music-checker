extern crate getopts;

use std::env;
use std::process;
use std::process::exit;
use std::fs::File;
use std::io::Read;
use getopts::Options;

fn main() {
    // Get commandline arguments into a getopts::Options object
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = getopts::Options::new();

    // List all possible arguments to be parsed
    opts.optflag("h", "help", "Print this help menu");
    opts.optflag("n", "new", "Create a new artist database");
    opts.optflag("a",
                 "check-all",
                 "Check all artists against the MusicBrainz database");
    opts.optflag("s",
                 "check-single",
                 "Check the least-recently-checked artist against the MusicBrainz database.");

    // Make getopts parse our arguments
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };

    if matches.opt_present("h") {
        // Print help and quit
        print_usage(program, opts);
        process::exit(0);
    }

	// Read the user agent from the `user-agent.txt` file:
	let user_agent = get_user_agent();

    if matches.opt_present("n") {
        // Make a new database and quit.
        println!("\nnew database\n");
        unimplemented!();
        process::exit(0);
    }

    // Check -a and -s; if match, check the artist(s) against MusicBrainz and quit.
    // NOTE: only one of (-a / -s) allowed
    let opt_a_present = matches.opt_present("a");
    let opt_s_present = matches.opt_present("s");
    if opt_a_present && opt_s_present {
        println!("ERROR: Only one of the (-a / -s) arguments is allowed!");
        process::exit(1);
    } else if opt_a_present {
        // Check all artists
        println!("\ncheck all artists\n");
        unimplemented!();
    } else if opt_s_present {
        // Check single least-recently-checked artist
        println!("\ncheck single artist\n");
        unimplemented!();
    } else {
        // Print missing releases
        println!("\nprint missing releases\n");
        unimplemented!();
    }
}

fn print_usage(program: String, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    let brief = format!("{}\n\nIf no options are given, this program will print out all missing \
                         releases.",
                        brief);
	let brief = format!("{}\n\nAlso, make sure to edit the `user-agent.txt` file.",
					    brief);
    print!("{}", opts.usage(&brief));
    exit(1);
}

fn get_user_agent() -> String {
	let mut f = File::open("user-agent.txt").unwrap();
	let mut s = String::new();
	f.read_to_string(&mut s).unwrap();
	if s.trim() == "EDIT ME".trim() {
		println!("ERROR: Edit the `user-agent.txt` file.");
		exit(1);
	}
	print!("{}", s);
	s
}
