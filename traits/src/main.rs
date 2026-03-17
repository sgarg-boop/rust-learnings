trait Summary {

    // will be written in impl summ for tweet
    fn summarize_author_name(&self) -> &str;  // dont take ownership

    // basically instead of writing one general string, we have to give author name then we can use that also
    fn summarize(&self) -> String {
        format!("Read more... for the author {}", self.summarize_author_name())
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

// now that shared fucntion we have to write fron our side
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        let content = format!(" News by {} is {}", self.author, self.content);
        content
    }

    fn summarize_author_name(&self) -> &str{
        self.author.as_str()
    }
}

struct Tweet {
    username: String,
    content: String,
    retweet: bool,
}

impl Summary for Tweet {
    // this is optional if we have default implementation
    // fn summarize(&self) -> String{
    //     let content = format!("Tweet by {} is {}", self.username, self.content);
    //     content
    // }

    fn summarize_author_name(&self) -> &str{
        self.username.as_str()
    }
}

fn main() {
    let tweet1 = Tweet {
        username: String::from("sgarg"),
        content: String::from("Hello World"),
        retweet: false,
    };

    let newsArticle1 = NewsArticle {
        headline: String::from("Hello World"),
        location: String::from("India"),
        author: String::from("sgarg"),
        content: String::from("Hello World"),
    };

    news_aggregator(&tweet1);
    news_aggregator(&newsArticle1);
}

fn news_aggregator(source: &impl Summary) {
    println!(" You are inside new aggregator");
    println!("Summary is {}", source.summarize());
}
