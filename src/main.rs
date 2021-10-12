mod config;
mod tweet;

use egg_mode::Token;
use rand::Rng;
use egg_mode::tweet::DraftTweet;
use std::io::Read;
use std::fs::File;

use crate::config::Config;
use crate::tweet::Tweet;

#[tokio::main]
async fn main() {
    
    // load twitter tokens from config file
    let config: Config = Config::load();

    // create token struct for egg-mode
    let consumer_token = egg_mode::KeyPair::new(config.api_key, config.api_secret);
    let access_token = egg_mode::KeyPair::new(config.access_token, config.access_secret);
    let token = egg_mode::Token::Access {
        consumer: consumer_token,
        access: access_token,
    };

    // read tweets file
    let mut file = File::open("./tweets/season1.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
    let tweets: Vec<Tweet> = serde_json::from_str(&buff).unwrap();

    // choose tweet
    let tweet_number = rand::thread_rng().gen_range(0..tweets.len());
    let tweet: &Tweet = &tweets[tweet_number];
    println!("{} {}-{}", tweet.message, tweet.id, tweet.tag);

    // publish tweet
    // let draft = DraftTweet::new("{} {}-{}", tweet.message, tweet.id, tweet.tag);
    // draft.send(&token).await.unwrap();
}
