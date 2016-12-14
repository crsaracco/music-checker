use std::result::Result;
use std::string::String;
use rusqlite;

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





//use rusqlite::{Connection, Error};

/*
CREATE TABLE IF NOT EXISTS artists (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT,
	musicbrainzId TEXT,
	lastChecked INTEGER
);
*/

/*
pub fn create_artists_table() -> Result<(), rusqlite::Error> {
    try!(conn.execute(
        "CREATE TABLE IF NOT EXISTS artists (
        	id INTEGER PRIMARY KEY AUTOINCREMENT,
        	name TEXT,
        	musicbrainzId TEXT,
        	lastChecked INTEGER
        )", &[])
    );
    Ok(())
}
*/

/*
pub fn get_rows() -> Result<Vec<i64>, Error> {
    let result: Vec<i64> = Vec::new();
    let conn = try!(Connection::open("test.sqlite"));
    let mut stmt = try!(conn.prepare("SELECT b FROM test"));

    let mut rows_iter = stmt.query_map(&[], |row| {
        let test: i64 = row.get(0);
        test
    }).unwrap();

    for row in rows_iter {
        println!("Test: {:?}", row.unwrap());
    }

    Ok(result)
}
*/
