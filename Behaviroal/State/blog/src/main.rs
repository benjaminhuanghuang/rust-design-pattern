use blog::Post;


fn main() {
 let mut post = Post::new();

 post.add_text("");

 post.request_re();


 post.approve();

 assert_eq!("", post.content())
}
