#[macro_use]
extern crate bitflags;

mod bus;
mod cartridge;
mod cpu;
mod sms;
mod vm;

use sms::SMS;

fn main() {
    let mut vm = SMS::default()
        .with_cartridge(Some("roms/zexall_sdsc.sms"))
        .build();

    vm.run();
}
