pub mod input_buffer_node;
pub use input_buffer_node::*;

use std::rc::Rc;
use std::cell::RefCell;
type Link = Option<Rc<RefCell<InputBufNode>>>;

/// A struct for storing the current state of user input with a linked-list for easy insertion.
pub struct InputBuf {
    idx: u8,
    line: u8,
    head: Link,
    current: Link,
    screen_width: u8,
}

impl InputBuf {
    /// Initialises a new `InputBuffer` with the set screen width; the amount of characters the screen can hold (not including the prompt) on one line.
    pub fn new(screen_width: u8) -> Self {
        Self {
            idx: 0,
            line: 0,
            screen_width,
            current: None,
            head: None,
        }
    }

    pub fn advance(&mut self) {
        let current = if let Some(x) = &self.current { x.clone() }
            else { return }; // Do nothing if current is none therefor there is no future.
        
        let borrowed = current.borrow();

        if let Some(x) = &borrowed.next {
            self.current = Some(x.clone()); // set new current
            self.idx += 1;
            if self.idx >= self.screen_width { // If cursor moves beyond screen increase line idx
                self.idx = 0;
                self.line += 1;
            }
        } // Does nothing if there is not next
    }

    pub fn un_advance(&mut self) {
        let current = if let Some(x) = &self.current { x.clone() }
            else { return }; // Do nothing if the current
    }
}