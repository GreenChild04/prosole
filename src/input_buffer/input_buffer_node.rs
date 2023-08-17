use std::rc::Rc;
use std::cell::RefCell;

type Link = Option<Rc<RefCell<InputBufNode>>>;

pub struct InputBufNode {
    pub next: Link,
    pub prev: Link,
    pub value: Box<str>,
}

impl InputBufNode {
    #[inline]
    pub fn is_head(&self) -> bool {
        self.prev.is_none()
    }

    #[inline]
    pub fn is_tail(&self) -> bool {
        self.next.is_none()
    }
}