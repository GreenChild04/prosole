use super::Link;

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