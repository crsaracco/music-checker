extern crate rusqlite;

use std::result::Result;
use std::string::String;

// TODO: use rusqlite::Connection

pub struct Database {
    conn: rusqlite::Connection,
}

impl Database {
    pub fn new(filename: String) -> Database {
        Database {
            conn: rusqlite::Connection::open(filename).unwrap(),
        }
    }

    pub fn create_artists_table(&self) -> Result<(), rusqlite::Error> {
        try!(self.conn.execute(
            "CREATE TABLE artists (
            	id INTEGER PRIMARY KEY AUTOINCREMENT,
            	name TEXT,
            	musicbrainzId TEXT,
            	lastChecked INTEGER
            )", &[])
        );
        Ok(())
    }

    pub fn create_releases_table(&self) -> Result<(), rusqlite::Error> {
        try!(self.conn.execute(
            "CREATE TABLE releases (
            	id INTEGER PRIMARY KEY AUTOINCREMENT,
            	artistId INTEGER,
            	title TEXT,
            	releaseGroupId TEXT,
            	releaseDate TEXT,
            	releaseType TEXT,
            	releaseStatus TEXT,
            	collected INTEGER, -- 0 = don't have, 1 = have, 2 = temporary ignore, 3 = permanent ignore
            	notes TEXT
            )", &[])
        );
        Ok(())
    }
}
