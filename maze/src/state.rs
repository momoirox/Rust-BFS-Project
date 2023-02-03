use std::collections::VecDeque;

use crate::maze::Cell;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct State {
    pub cell: Cell,
    pub keys: VecDeque<KeyState>,
}
impl State {
    pub fn init(cell: Cell, keys: VecDeque<KeyState>) -> State {
        State {
            cell: cell,
            keys: keys,
        }
    }
    pub fn new(cell: Cell, keys: VecDeque<KeyState>) -> State {
        State {
            cell: cell,
            keys: keys,
        }
    }
}
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct KeyState {
    pub coordinates: (u32, u32),
    pub used: bool,
}
impl KeyState {
    pub fn new(coordnates: (u32, u32), used: bool) -> KeyState {
        KeyState {
            coordinates: coordnates,
            used: used,
        }
    }
}
