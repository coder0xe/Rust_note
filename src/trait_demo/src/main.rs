use trait_demo::NewsArtical;
use trait_demo::Summary;
use trait_demo::Tweet;

fn main() {
    let t = Tweet {
        username : String::from("dqr"),
        content : String::from("brother YUAN is a thief"),
        reply : false,
        retweet : false,
    };
    println!("{}", t.summarize());
    let n = NewsArtical {
        headline: String::from("BeiJing"),
        location: String::from("QiTaiHe"),
        author: String::from("ZhiYuan Qin"),
        content: String::from("ok somebody people"),
    };
    println!("{}", n.summarize());
}
