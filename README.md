# Music Checker

My typical project when I'm learning a new language.

This program will check your music collection against the [MusicBrainz](https://musicbrainz.org/) database, and will tell you if you're missing any releases. It's useful for keeping up-to-date with your favorite artists to make sure you don't miss anything new.

For now, your "collection" is just some metadata inside a sqlite database, and the user has to manually add artists and confirm/ignore releases via sqlite. Maybe I'll write some interface through the rust program eventually.

### Dependencies

You need to be running a nightly version of Rust, since this project is using the `serde` crate using the "nightly" method.

### TODOs:
 - `musicbrainz.rs`: `check_release_group_official()` - need to make a Release type, get all Releases from a given ReleaseGroup, and check if any of them are official.
 - Start using `rustfmt`
 - `grep -Ri "todo" .`

### License

MIT. See the `LICENSE` file for more info.
