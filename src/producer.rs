use std::cell::Cell;
use std::rc::Rc;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ProducerState {
    Started,
    Closed,
}

pub struct Producer {
    func: Box<Fn(ProducerState)>,
    state: Cell<ProducerState>,
}

impl Producer {
    pub fn from_func(func: Box<Fn(ProducerState)>) -> Producer {
        Producer {
            func: func,
            state: Cell::new(ProducerState::Started),
        }
    }

    pub fn new() -> Producer {
        Producer::from_func(Box::new(|_| {}))
    }

    pub fn is_closed(&self) -> bool {
        self.state.get() == ProducerState::Closed
    }

    pub fn close(&self) {
        self.state.set(ProducerState::Closed);
    }
}
