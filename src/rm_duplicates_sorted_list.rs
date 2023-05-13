// If a struct, enum or union were to directly or indirectly contain itself,
// without using some kind of pointer,
// the size would have to be infinite,
// which isn't possible.
// By using Option<Box<T>>, you only allocate more space when you actually need it.

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

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }
    let mut h: Option<Box<ListNode>> = head;
    let mut p1: &mut Box<ListNode> = h.as_mut().unwrap();
    while let Some(p2) = p1.next.as_mut() {
        if p1.val == p2.val {
            p1.next = p2.next.take();
        } else {
            p1 = p1.next.as_mut().unwrap();
        }
    }
    h
}

pub fn run() {
    let node = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        })),
    }));

    println!("{:?}", delete_duplicates(node));
}
