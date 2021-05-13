#![no_std]
#![no_main]

psp::module!("button tester2", 2, 0);

use psp::sys::{SceCtrlData, CtrlButtons, CtrlMode};

pub fn psp_main() {
    psp::enable_home_button();
    // this is where the main data is stored
unsafe{
    psp::sys::sceCtrlSetSamplingCycle(0);
    psp::sys::sceCtrlSetSamplingMode(CtrlMode::Analog);

    let ctrl_data = &mut SceCtrlData::default();
    // this reads the imput data
    loop {
        psp::sys::sceCtrlReadBufferPositive(ctrl_data, 1);
        if ctrl_data.buttons.contains(CtrlButtons::CROSS) {
            psp::dprintln!("PRESSED CROSS");
        }
        if ctrl_data.buttons.contains(CtrlButtons::CIRCLE) {
            psp::dprintln!("PRESSED CIRCLE");
        }
        if ctrl_data.buttons.contains(CtrlButtons::SQUARE) {
            psp::dprintln!("PRESSED SQUARE");
        }
        if ctrl_data.buttons.contains(CtrlButtons::TRIANGLE) {
            psp::dprintln!("PRESSED TRIANGLE");
       }
        if ctrl_data.buttons.contains(CtrlButtons::LTRIGGER) {
            psp::dprintln!("PRESSED LEFT TRIGGER");
        }
        if ctrl_data.buttons.contains(CtrlButtons::RTRIGGER) {
            psp::dprintln!("PRESSED RIGHT TRIGGER");
        }
    }
}
}