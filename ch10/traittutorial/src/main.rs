mod summary;

use crate::summary::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Champion"),
        location: String::from("USA"),
        author: String::from("Ice"),
        content: String::from("Hello"),
    };

    println!("News Article: {}", article.summarize());
}
