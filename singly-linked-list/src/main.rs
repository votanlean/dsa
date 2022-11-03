fn main() {
    let mut obj = MyLinkedList::new();
    obj.add_at_index(1, 0);
    let ret_1: i32 = obj.get(0);
    assert_eq!(ret_1, -1);
    obj.add_at_head(3);
    obj.add_at_head(1);
    obj.add_at_index(1, 2);
    obj.add_at_tail(4);
    obj.delete_at_index(3);
    assert_eq!(obj.get(0), 1);
    assert_eq!(obj.get(1), 2);
    assert_eq!(obj.get(2), 3);
    assert_eq!(obj.get(3), -1);
}

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}
#[derive(Default)]
struct MyLinkedList {
    head: Option<Box<Node>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    fn new() -> Self {
        Default::default()
    }

    fn get(&self, index: i32) -> i32 {
        let mut cur = match self.head {
            Some(ref a) => a,
            None => return -1,
        };
        let mut idx_cur = 0;
        while idx_cur < index {
            if let Some(ref next) = cur.next {
                cur = next;
                idx_cur += 1;
            } else {
                return -1;
            };
        }
        cur.val
    }

    fn add_at_head(&mut self, val: i32) {
        self.head = Some(Box::new(Node {
            val: val,
            next: self.head.take(),
        }))
    }

    fn add_at_tail(&mut self, val: i32) {
        let mut cur = match self.head {
            Some(ref mut a) => a,
            None => {
                self.head = Some(Box::new(Node { val, next: None }));
                return;
            }
        };
        while let Some(ref mut next) = cur.next {
            cur = next;
        }
        cur.next = Some(Box::new(Node { val, next: None }));
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        let mut dummy_head = Box::new(Node {
            val: 0,
            next: self.head.take(),
        });
        let mut idx = 0;
        let mut cur = &mut dummy_head;
        while idx < index {
            if let Some(ref mut next) = cur.next {
                cur = next;
            } else {
                return;
            }
            idx += 1;
        }
        cur.next = Some(Box::new(Node {
            val: val,
            next: cur.next.take(),
        }));
        self.head = dummy_head.next;
    }

    fn delete_at_index(&mut self, index: i32) {
        let mut dummy_head = Box::new(Node {
            val: 0,
            next: self.head.take(),
        });
        let mut idx = 0;
        let mut cur = &mut dummy_head;
        while idx < index {
            if let Some(ref mut next) = cur.next {
                cur = next;
            }
            idx += 1;
        }
        cur.next = cur.next.take().and_then(|a| a.next);
        self.head = dummy_head.next;
    }
}
