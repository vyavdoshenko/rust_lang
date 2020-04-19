use default_trait_impl::Summary;

fn main() {
    let tweet = returns_summarizable();

    println!("1 new tweet: {}", tweet.summarize());

    default_trait_impl::notify(tweet);
}

fn returns_summarizable() -> impl Summary {
    default_trait_impl::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}