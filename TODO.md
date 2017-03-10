# TODO

 - Main
    - ~~Read file for user agent~~
    - Read commandline arguments
       - ~~help --> prints help menu~~
       - new --> create a new database *(for now, create a hardcoded sqlite database name)*
       - check-all --> check all artists
       - check-single --> check single artist (least-recently-checked for now)
       - By default (no flags), prints missing releases.
 - MusicChecker
    - Check single artist by id
    - **[NICE-TO-HAVE]** Check single artist by name (get id from database, use id func)
    - Check single artist by least recent (get id from database, use id func)
    - Check all artists (get all ids from database, use id func)
    - Print all missing releases
    - Create a new database
 - Database
    - [pub] Create new database
    - [pub] Get id of least-recently-checked artist
    - **[NICE-TO-HAVE]** [pub] Get id of artist by name
    - [priv] Check if we have a release group in the database already
    - [pub] Store a release group (use above check)
    - [pub] update the lastChecked value for this artist (use in any artist check)
    - [pub] get all missing releases
 - MusicBrainz
    - NOTE: make sure it's rate-limited to follow API guidelines
    - NOTE: make sure you have an API user agent before you do anything
    - [priv] make a function that does "loop while we don't have all the results" (if possible?)
    - [pub] Fetch an artist's release groups; filter by Official AND (Album, ..., or ...)

----

### TODO priority queue

 - Make a "MusicChecker" struct with stubbed out functions (check single artist by id, check single artist by least recent, check all artists, print missing, create database) - use these stubbed out functions in main instead of previous `unimplemented!()`
 - Make a "Database" struct
