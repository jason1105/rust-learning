type RNode = Option<Box<Node>>;

struct List {
    head: RNode,
}

struct Node {
    val: u32,
    next: RNode,
}

impl List {

    pub fn push(&mut self, val: u32) {
        let mut new_node = Box::new(Node{val: val, next:self.head.take()});
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<u32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }

    pub fn peek(&self) -> Option<&u32> {
        // Some(self.head.as_ref().unwrap().val)
        self.head.as_ref().map(|node| &node.val)
    }

    pub fn peek_mut(&mut self) -> Option<&mut u32> {
        self.head.as_mut().map(|node| &mut node.val)
    }
    
}
