use std::{cell::RefCell, rc::Rc, vec};

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Default)]
#[allow(dead_code)]
struct Node {
    next: Vec<Link>,
    pub offset: u64,
    pub command: String,
}

impl Node {
    fn new(links: Vec<Link>, offset: u64, command: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            next: links,
            offset: offset,
            command: command,
        }))
    }
}

#[allow(dead_code)]
#[derive(Clone)]
struct SkipList {
    head: Link,
    tails: Vec<Link>,
    max_level: usize,
    pub length: u64,
}

impl SkipList {
    #[allow(dead_code)]
    pub fn new(max_level: usize) -> Self {
        Self {
            head: None,
            tails: vec![None; max_level + 1],
            max_level: max_level,
            length: 0,
        }
    }
    #[allow(dead_code)]
    fn get_level(&self) -> usize {
        let mut n = 0;
        while rand::random::<bool>() && n < self.max_level {
            n += 1;
        }
        n
    }
    #[allow(dead_code)]
    pub fn append(&mut self, offset: u64, value: String) {
        let level = 1 + if self.head.is_none() {
            self.max_level
        } else {
            self.get_level()
        };

        let new = Node::new(vec![None; level], offset, value);

        for i in 0..level {
            if let Some(old) = self.tails[i].take() {
                let next = &mut old.borrow_mut().next;
                next[i] = Some(new.clone());
            }
        }

        if self.head.is_none() {
            self.head = Some(new.clone());
        }
        self.length += 1;
    }

    #[allow(dead_code)]
    pub fn find(&self, offset: u64) -> Option<String> {
        match self.head {
            Some(ref head) => {
                let mut start_level = self.max_level;
                let node = head.clone();
                let mut result = None;
                loop {
                    if node.borrow().next[start_level].is_some() {
                        break;
                    }
                    start_level -= 1;
                }
                let mut n = node;
                for level in (0..=start_level).rev() {
                    loop {
                        let next = n.clone();
                        match next.borrow().next[level] {
                            Some(ref next) if next.borrow().offset <= offset => n = next.clone(),
                            _ => break,
                        };
                    }
                    if n.borrow().offset == offset {
                        let tmp = n.borrow();
                        result = Some(tmp.command.clone());
                        break;
                    }
                }
                result
            }
            None => None,
        }
    }
    #[allow(dead_code)]
    fn iter_level(&self, level: usize) -> ListIterator {
        ListIterator::new(self.head.clone(), level)
    }
}

impl IntoIterator for SkipList {
    type Item = (u64, String);
    type IntoIter = ListIterator;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self.head, 0)
    }
}

#[allow(dead_code)]
pub struct ListIterator {
    current: Option<Rc<RefCell<Node>>>,
    level: usize,
}

impl ListIterator {
    fn new(start_at: Option<Rc<RefCell<Node>>>, level: usize) -> Self {
        Self {
            current: start_at,
            level: level,
        }
    }
}

impl Iterator for ListIterator {
    type Item = (u64, String);

    fn next(&mut self) -> Option<(u64, String)> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some((current.offset, current.command.clone()));
                current.next[self.level].clone()
            },
            _ => None
        };
        result
    }
}

impl std::fmt::Debug for SkipList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.head {
            Some(ref _head) => {
                for level in (0..=self.max_level).rev() {
                    let _ = write!(f, "{}: ", level);
                    for n in self.iter_level(level) {
                        let _ = write!(f, "[{}] ", n.0);
                    }
                    let _ = writeln!(f, "");
                }
                Ok(())
            }
            None => write!(f, "The list is empty: []")
        }
    }
}