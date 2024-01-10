use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }

    /// list1 = A -> B -> C -> D
    /// list2 = tail(list1) = B -> C -> D
    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_deref().and_then(|node| node.next.clone()),
        }
    }

    /// Peek
    pub fn head(&self) -> Option<&T> {
        self.head.as_deref().map(|node| &node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(rc_node) = head {
            if let Ok(mut node) = Rc::try_unwrap(rc_node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct ListIter<'a, T>(Option<&'a Node<T>>);

impl<'a, T: Copy> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.map(|node| {
            self.0 = node.next.as_deref(); // Option<Rc<Node<T>>> -> Option<&Node<T>>
            &node.elem
        })
    }
}

impl<T: Copy> List<T> {
    // lifetime bounds can be elide
    fn iter<'a>(&'a self) -> ListIter<'a, T> {
        ListIter(self.head.as_deref())
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
