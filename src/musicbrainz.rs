use std::string::String;
use std::io::Read;
use std::time::Duration;
use hyper::Client;
use hyper::header::UserAgent;
use ratelimit::Ratelimit;
use serde_json;

// TODO: rate-limiting per API rules

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ReleaseGroup {
    pub id: String,
    pub title: String,
    #[serde(rename="primary-type")]
    pub primary_type: String,
    #[serde(rename="first-release-date")]
    pub first_release_date: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ReleaseGroups {
    #[serde(rename="release-groups")]
    release_groups: Vec<ReleaseGroup>,
}

pub struct MusicBrainz {
    user_agent: String,
    rate_limit: Ratelimit,
}

impl MusicBrainz {
    pub fn new(useragent: String) -> MusicBrainz {
        let ratelimit = Ratelimit::configure()
            .capacity(1) //number of tokens the bucket will hold
            .quantum(1) //add one token per interval
            .interval(Duration::new(2, 0)) //add quantum tokens every 1 second
            .build();
        MusicBrainz {
            user_agent: useragent,
            rate_limit: ratelimit
        }
    }

    pub fn fetch_release_groups(&mut self, artist_id: String) -> Vec<ReleaseGroup> {
        let mut result_vec: Vec<ReleaseGroup> = Vec::new();

        // TODO: make a function to do this "looping while we don't have all the results" thing
        let mut counter = 0;
        let num_objects = 100;
        loop {
            let url = format!("http://musicbrainz.org/ws/2/release-group?artist={}&fmt=json&limit={}&offset={}", artist_id, num_objects, counter);
            let url_data = self.fetch_url(url);
            let mut json_data: ReleaseGroups = serde_json::from_str(&url_data).unwrap();

            let result_length = json_data.release_groups.len();
            result_vec.append(&mut json_data.release_groups);

            println!("{}", result_length);

            if result_length < num_objects {
                break;
            }
            counter += num_objects
        }

        println!("{}", result_vec.len());

        result_vec
    }

    // TODO: Make a function "fetch_releases(release_group_id) -> Vec<Release>"

    pub fn check_release_group_official(&mut self, release_group_id: String) -> bool {
        let mut counter = 0;
        let num_objects = 100;
        loop {
            let url = format!("http://musicbrainz.org/ws/2/release?release-group={}&fmt=json&limit={}&offset={}", release_group_id, num_objects, counter);
            let url_data = self.fetch_url(url);
            println!("{}", url_data);
        }
        true
    }

    fn fetch_url(&mut self, url: String) -> String {
        self.rate_limit.block(1);
        let mut response_body = String::new();
        let client = Client::new();

        println!("{}", url);
        let user_agent = self.user_agent.clone();
        let mut res = client.get(&url).header(UserAgent(user_agent)).send().unwrap();
        println!("Status Code: {}", res.status);
        res.read_to_string(&mut response_body).unwrap();
        response_body
    }
}
