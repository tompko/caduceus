use bus::Bus;
use cpu::Cpu;

pub struct VM {
    bus: Bus,
    cpu: Cpu,
}

impl VM {
    pub fn new(bus: Bus) -> VM {
        VM {
            bus: bus,
            cpu: Cpu::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.step(&mut self.bus);
        }
    }
}
