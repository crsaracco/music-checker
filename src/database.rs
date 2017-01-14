extern crate rusqlite;

use std::result::Result;
use std::string::String;

// TODO: use rusqlite::Connection

pub struct Database {
    conn: rusqlite::Connection,
}

#[derive(Debug)]
pub struct Artist {
    id: i32,
    name: String,
    musicbrainz_id: String,
    last_checked: i64
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

    pub fn get_least_recently_checked_artist(&self) -> Result<Artist, rusqlite::Error> {
        let mut stmt = self.conn.prepare("SELECT id, name, musicbrainzId, lastChecked FROM artists ORDER BY lastChecked ASC LIMIT 1").unwrap();
        let mut artist_iter = stmt.query_map(&[], |row| {
            Artist {
                id: row.get(0),
                name: row.get(1),
                musicbrainz_id: row.get(2),
                last_checked: row.get(3)
            }
        }).unwrap();

        for artist in artist_iter {
            //println!("Found artist {:?}", artist);
            return artist; // Return the first one we find
        }

        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}
