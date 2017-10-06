mod executor;
mod io;
mod operands;
mod operations;
mod state;

use self::state::State;
use self::executor::Executor;
use super::bus::Bus;

pub struct Cpu {
    state: State,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            state: State::default(),
        }
    }

    pub fn step(&mut self, b: &mut Bus) {
        let executor = Executor(&mut self.state, b);

        operations::visit(executor);
    }
}
