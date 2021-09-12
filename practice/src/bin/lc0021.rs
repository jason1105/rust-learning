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
        let mut ans_p = &mut ans;
        loop {
            match (l1.take(), l2.take()) {
                (Some(mut a), Some(mut b)) => {
                    let ans_next;
                    if a.val < b.val {
                        l1 = a.next.take();
                        l2 = Some(b);
                        ans_next = Some(a);
                    } else {
                        l1 = Some(a);
                        l2 = b.next.take();
                        ans_next = Some(b);
                    }
                    ans_p.as_mut().unwrap().next = ans_next;
                    ans_p = &mut ans_p.as_mut().unwrap().next;
                }
                (Some(a), None) => {
                    ans_p.as_mut().unwrap().next = Some(a);
                    break;
                }
                (None, Some(b)) => {
                    ans_p.as_mut().unwrap().next = Some(b);
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
