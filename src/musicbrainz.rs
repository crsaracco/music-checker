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
    id: String,
    title: String,
    #[serde(rename="primary-type")]
    primary_type: String,
    #[serde(rename="first-release-date")]
    first_release_date: Option<String>,
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

        let mut counter = 0;
        let num_objects = 25;
        loop {
            let test = self.fetch_url(format!("http://musicbrainz.org/ws/2/release-group?artist={}&fmt=json&limit={}&offset={}", artist_id, num_objects, counter));
            let mut json_data: ReleaseGroups = serde_json::from_str(&test).unwrap();

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

    fn fetch_url(&mut self, url: String) -> String {
        self.rate_limit.block(1);
        let mut response_body = String::new();
        let client = Client::new();

        println!("{}", url);
        let user_agent = self.user_agent.clone();
        let mut res = client.get(&url).header(UserAgent(user_agent)).send().unwrap();

        res.read_to_string(&mut response_body).unwrap();
        response_body
    }
}
