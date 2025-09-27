/*
    single linked list merge
    This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        // node.next = None; // 有用吗？？
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    // pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self {
    //     //TODO
    //     let mut list = LinkedList::new();
    //     while list_a.length > 0 && list_b.length > 0 {
    //         let a = unsafe { (*list_a.start.unwrap().as_ptr()).val };
    //         let b = unsafe { (*list_b.start.unwrap().as_ptr()).val };
    //         if a > b {
    //             list.add(b);
    //             list_b.start = unsafe { &(*list_b.start.unwrap().as_ptr()).next };
    //             list_b.length -= 1;
    //         } else {
    //             list.add(a);
    //             list_a.start = unsafe { &(*list_a.start.unwrap().as_ptr()).next };
    //
    //             list_a.length -= 1;
    //         }
    //     }
    //
    //     while list_a.length > 0 {
    //         let a = unsafe { (*list_a.start.unwrap().as_ptr()).val };
    //         list.add(a);
    //         list_a.start = unsafe { &(*list_a.start.unwrap().as_ptr()).next };
    //         list_a.length -= 1;
    //     }
    //     while list_b.length > 0 {
    //         let b = unsafe { (*list_b.start.unwrap().as_ptr()).val };
    //         list.add(b);
    //         list_b.start = unsafe { &(*list_b.start.unwrap().as_ptr()).next };
    //         list_b.length -= 1;
    //     }
    //     list
    // }
}
impl<T: Ord> LinkedList<T> {
    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self {
        let mut merged = LinkedList::new();

        // 我们可以通过 while 循环，从 list_a 和 list_b 的头部取出最小值
        while let (Some(a_ptr), Some(b_ptr)) = (list_a.start, list_b.start) {
            let a_ref = unsafe { a_ptr.as_ref() };
            let b_ref = unsafe { b_ptr.as_ref() };

            // 比较头节点的值
            if a_ref.val <= b_ref.val {
                // 取出 list_a 的头节点
                let node = list_a.pop_front_node();
                merged.append_node(node);
            } else {
                // 取出 list_b 的头节点
                let node = list_b.pop_front_node();
                merged.append_node(node);
            }
        }

        // 将剩余节点全部加入
        while list_a.length > 0 {
            let node = list_a.pop_front_node();
            merged.append_node(node);
        }

        while list_b.length > 0 {
            let node = list_b.pop_front_node();
            merged.append_node(node);
        }

        merged
    }

    // 私有辅助函数：弹出第一个节点，返回其 Box<Node<T>>
    fn pop_front_node(&mut self) -> Box<Node<T>> {
        if let Some(node_ptr) = self.start {
            let boxed_node = unsafe { Box::from_raw(node_ptr.as_ptr()) };
            self.start = boxed_node.next;
            if self.start.is_none() {
                self.end = None;
            }
            self.length -= 1;
            boxed_node
        } else {
            panic!("pop_front_node called on empty list");
        }
    }

    // 私有辅助函数：添加一个已有的节点（转移所有权）
    fn append_node(&mut self, mut node: Box<Node<T>>) {
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }
}
impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}
