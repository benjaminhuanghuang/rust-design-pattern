
pub struct Post {
  state: Option<Box<dyn, State>>,
  // internal field
  constent: String
}

impl Post {
  pub fn new () -> Self {
    Post {
      state: Some(Box::new(Draft{})),
      constent: String::new(),
    }
  }

  pub fn add_text(&mut self, text: &str) {
    self.constent.push_str(text);
  }

  pub fn content(&self) -> &str {
    ""
  }

  pub fn request_review(&mut self) {
    if let Som(state) = self.state.take(){
      self.state = Some(state.request_review());
    }
  }
}

trait state {
  fn request_review(self : Box<Self>) ->Box<dyn State> {
    Box::new(PendingReview{})
  }
}


strcut Drafe {}

impl State for Draft{

}
struct PendingReview {

}

impl State for PendingReview {
  fn request_review(self : Box<Self>) ->Box<dyn State> {
    Box::new(PendingReview{})
  }
}