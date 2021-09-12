struct Solution;

fn main() {
    let mut n1 = ListNode::new(3);
    let mut n2 = ListNode::new(5);
    let n4 = ListNode::new(6);
    n2.next = Some(Box::new(n4));
    n1.next = Some(Box::new(n2));
    let l1 = Some(Box::new(n1));

    let mut n1 = ListNode::new(1);
    let mut n3 = ListNode::new(2);
    let n4 = ListNode::new(4);
    n3.next = Some(Box::new(n4));
    n1.next = Some(Box::new(n3));
    let l2 = Some(Box::new(n1));

    println!("l1 = {:?}", l1);
    println!("l2 = {:?}", l2);

    let ans = Solution::merge_two_lists(l1, l2);
    println!("ans = {:?}", ans);
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut ans = Some(Box::new(ListNode::new(0)));
        let mut anss = &mut ans;
        loop {
            match (l1.as_mut().take(), l2.as_mut()) {
                (Some(a), Some(b)) => {
                    if a.val < b.val {
                        anss.as_mut().unwrap().next = l1.take();
                        l1 = anss.as_mut().unwrap().next.as_mut().unwrap().next.take();
                    } else {
                        anss.as_mut().unwrap().next = l2.take();
                        l2 = anss.as_mut().unwrap().next.as_mut().unwrap().next.take();
                    }
                    anss = &mut anss.as_mut().unwrap().next;
                }
                (Some(a), None) => {
                    anss.as_mut().unwrap().next = l1.take();
                    break;
                }
                (None, Some(b)) => {
                    anss.as_mut().unwrap().next = l2.take();
                    break;
                }
                _ => {
                    break;
                }
            }
        }
        ans.unwrap().next
    }
}
