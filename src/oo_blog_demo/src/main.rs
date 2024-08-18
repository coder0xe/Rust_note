use oo_blog_demo::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("李国庆是色懒");
    println!("{}", post.content());
    post.request_review();
    println!("{}", post.content());
    post.approve();
    println!("{}", post.content());
}