
// design pattern: State

/* Post 对象有不同的状态
 Draft: 草稿状态.    调用 request_review() 进入下一个状态
 PendingReview: 等待审核状态.   调用 approve() 进入下一个状态
 Published: 已发布(审核ok)状态.   
*/
#[allow(unused_variables, unused_mut, unused_assignments)]
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

#[allow(unused_variables, unused_mut, unused_assignments)]
impl Post {
    pub fn new() -> Self {
        Post {
            state: Some(Box::new(Draft{})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(&mut self) {
        if let Some(local_state) = self.state.take() {
           self.state = Some(local_state.request_review());
        }
    }
    pub fn approve(&mut self) {
        if let Some(local_state) = self.state.take() {
            self.state = Some(local_state.approve());
         }
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
}

// 所有状态应该具有的方法. 每个具体状态内部实现这个三个方法.
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str;
}

// 三种具体状态
struct Draft {} // "草稿" 状态. 等待提交审核
struct PendingReview {} // "待审核" 状态.  等待审核
struct Published {} // "已发布" 状态

// "草稿" 状态
#[allow(unused_variables, unused_mut, unused_assignments)]
impl State for Draft {

    // 提交审查后, 状态转为等待审核
    fn request_review(self: Box<Self>)  -> Box<dyn State> {
        Box::new(PendingReview{})
    }

    // 目前是草稿状态, 不能直接同意, 所以 Blog 的状态不能改变, 还是草稿
    fn approve(self: Box<Self>)  -> Box<dyn State> {
        self
    }

    // 草稿状态的 Blog 不能发表, 所以返回空串
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// "待审核" 状态
#[allow(unused_variables, unused_mut, unused_assignments)]
impl State for PendingReview {

    // blog 等待审核, 再次提交审核并不能改变 blog 的状态, 所以仍然是"待审核"状态
    fn request_review(self: Box<Self>)  -> Box<dyn State> {
        self
    }

    // 同意, 则 blog 可以发布了, 状态改为 "已发布"
    fn approve(self: Box<Self>)  -> Box<dyn State> {
        Box::new(Published{})
    }

    // blog 等待审核, 还没定稿, 不能发表, 返回空串
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// "已发布" 状态
#[allow(unused_variables, unused_mut, unused_assignments)]
impl State for Published {

    // blog 已发布, 没有请求审核的必要, 保持现有状态
    fn request_review(self: Box<Self>)  -> Box<dyn State> {
        self
    }

    // blog 已发布, 没有审核的必要, 保持现有状态
    fn approve(self: Box<Self>)  -> Box<dyn State> {
        self
    }

    // blog 已发布, 可以看到内容.
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}