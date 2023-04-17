use std::fmt::{Display, Debug};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

/*impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}*/

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// impl Summary for NewsArticle {}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news {}", item.summarize());
}

// example of where clause
fn some_function<T, U>(t: &T, u: &U) -> i32
where 
    T: Display + Clone,
    U: Clone + Debug,   {
    // code goes here
    23 // to return an int meanwhile...
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("johan_cruyff"),
        content: String::from(
            "I read the book \"My Turn - A life of Football\", Johan Cruyff",
            ),
            reply: false,
            retweet: false,
    }
}

fn tweet_summary_impl() -> impl Summary {
    Tweet {
        username: String::from("messi_feats"),
        content: String::from(
            "Messi scored 91 goals in a calendar year. This is a \
            record no one will beat."
        ),
        reply: false,
        retweet: false,
    }
}

pub fn traits_main() {

   let tweet = Tweet {
        username: String::from("messi_feats"),
        content: String::from(
            "Messi scored 91 goals in a calendar year. This is a \
            record no one will beat."
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // ---------------------------------
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pitsburgh, Pa, Usa"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pitsburgh Penguins once again are the best \
            hockey team in the NHL and thus, world",
        ),
    };

    // println!("New article available! {}", article.summarize());

}



