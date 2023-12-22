#![no_std]


use crate::keyboard_layout::{ALT_GR, ISO_KEY, ISO_REPLACEMENT, SHIFT};

pub const KEY_LEFT_CTRL: u8 = 0x80;
pub const KEY_LEFT_SHIFT: u8 = 0x81;
pub const KEY_LEFT_ALT: u8 = 0x82;
pub const KEY_LEFT_GUI: u8 = 0x83;
pub const KEY_RIGHT_CTRL: u8 = 0x84;
pub const KEY_RIGHT_SHIFT: u8 = 0x85;
pub const KEY_RIGHT_ALT: u8 = 0x86;
pub const KEY_RIGHT_GUI: u8 = 0x87;

// Misc keys
pub const KEY_UP_ARROW: u8 = 0xDA;
pub const KEY_DOWN_ARROW: u8 = 0xD9;
pub const KEY_LEFT_ARROW: u8 = 0xD8;
pub const KEY_RIGHT_ARROW: u8 = 0xD7;
pub const KEY_BACKSPACE: u8 = 0xB2;
pub const KEY_TAB: u8 = 0xB3;
pub const KEY_RETURN: u8 = 0xB0;
pub const KEY_MENU: u8 = 0xED;
// "Keyboard Application" in USB standard
pub const KEY_ESC: u8 = 0xB1;
pub const KEY_INSERT: u8 = 0xD1;
pub const KEY_DELETE: u8 = 0xD4;
pub const KEY_PAGE_UP: u8 = 0xD3;
pub const KEY_PAGE_DOWN: u8 = 0xD6;
pub const KEY_HOME: u8 = 0xD2;
pub const KEY_END: u8 = 0xD5;
pub const KEY_CAPS_LOCK: u8 = 0xC1;
pub const KEY_PRINT_SCREEN: u8 = 0xCE;
// Print Screen / SysRq
pub const KEY_SCROLL_LOCK: u8 = 0xCF;
pub const KEY_PAUSE: u8 = 0xD0; // Pause / Break

// Numeric keypad
pub const KEY_NUM_LOCK: u8 = 0xDB;
pub const KEY_KP_SLASH: u8 = 0xDC;
pub const KEY_KP_ASTERISK: u8 = 0xDD;
pub const KEY_KP_MINUS: u8 = 0xDE;
pub const KEY_KP_PLUS: u8 = 0xDF;
pub const KEY_KP_ENTER: u8 = 0xE0;
pub const KEY_KP_1: u8 = 0xE1;
pub const KEY_KP_2: u8 = 0xE2;
pub const KEY_KP_3: u8 = 0xE3;
pub const KEY_KP_4: u8 = 0xE4;
pub const KEY_KP_5: u8 = 0xE5;
pub const KEY_KP_6: u8 = 0xE6;
pub const KEY_KP_7: u8 = 0xE7;
pub const KEY_KP_8: u8 = 0xE8;
pub const KEY_KP_9: u8 = 0xE9;
pub const KEY_KP_0: u8 = 0xEA;
pub const KEY_KP_DOT: u8 = 0xEB;

// Function keys
pub const KEY_F1: u8 = 0xC2;
pub const KEY_F2: u8 = 0xC3;
pub const KEY_F3: u8 = 0xC4;
pub const KEY_F4: u8 = 0xC5;
pub const KEY_F5: u8 = 0xC6;
pub const KEY_F6: u8 = 0xC7;
pub const KEY_F7: u8 = 0xC8;
pub const KEY_F8: u8 = 0xC9;
pub const KEY_F9: u8 = 0xCA;
pub const KEY_F10: u8 = 0xCB;
pub const KEY_F11: u8 = 0xCC;
pub const KEY_F12: u8 = 0xCD;
pub const KEY_F13: u8 = 0xF0;
pub const KEY_F14: u8 = 0xF1;
pub const KEY_F15: u8 = 0xF2;
pub const KEY_F16: u8 = 0xF3;
pub const KEY_F17: u8 = 0xF4;
pub const KEY_F18: u8 = 0xF5;
pub const KEY_F19: u8 = 0xF6;
pub const KEY_F20: u8 = 0xF7;
pub const KEY_F21: u8 = 0xF8;
pub const KEY_F22: u8 = 0xF9;
pub const KEY_F23: u8 = 0xFA;
pub const KEY_F24: u8 = 0xFB;

const HID_REPORT_DESCRIPTOR: [u8; 47] = [
    // Keyboard
    0x05, 0x01, // USAGE_PAGE (Generic Desktop)
    0x09, 0x06, // USAGE (Keyboard)
    0xa1, 0x01, // COLLECTION (Application)
    0x85, 0x02, // REPORT_ID (2)
    0x05, 0x07, // USAGE_PAGE (Keyboard)
    0x19, 0xe0, // USAGE_MINIMUM (Keyboard LeftControl)
    0x29, 0xe7, // USAGE_MAXIMUM (Keyboard Right GUI)
    0x15, 0x00, // LOGICAL_MINIMUM (0)
    0x25, 0x01, // LOGICAL_MAXIMUM (1)
    0x75, 0x01, // REPORT_SIZE (1)
    0x95, 0x08, // REPORT_COUNT (8)
    0x81, 0x02, // INPUT (Data,Var,Abs)
    0x95, 0x01, // REPORT_COUNT (1)
    0x75, 0x08, // REPORT_SIZE (8)
    0x81, 0x03, // INPUT (Cnst,Var,Abs)
    0x95, 0x06, // REPORT_COUNT (6)
    0x75, 0x08, // REPORT_SIZE (8)
    0x15, 0x00, // LOGICAL_MINIMUM (0)
    0x25, 0x73, // LOGICAL_MAXIMUM (115)
    0x05, 0x07, // USAGE_PAGE (Keyboard)
    0x19, 0x00, // USAGE_MINIMUM (Reserved (no event indicated))
    0x29, 0x73, // USAGE_MAXIMUM (Keyboard Application)
    0x81, 0x00, // INPUT (Data,Ary,Abs)
    0xc0, // END_COLLECTION
];

struct KeyReport {
    modifiers: u8,
    reserved: u8,
    keys: [u8; 6],
}

struct HIDSubDescriptor<'a> {
    data: &'a [u8],
}

struct HID;

//impl<'a> HID {
//    fn append_descriptor(&mut self, descriptor: &HIDSubDescriptor<'a>) {
//        // Implementation for AppendDescriptor
//        // You may want to add your own logic here.
//        println!("AppendDescriptor: {:?}", descriptor.data);
//    }
//
//    fn send_report(&mut self, report_id: u8, keys: &KeyReport) {
//        // Implementation for SendReport
//        // You may want to add your own logic here.
//        println!("SendReport (Report ID {}): {:?}", report_id, keys);
//    }
//}

struct Keyboard<'a> {
    keyreport: KeyReport,
    asciimap: &'a [u8],
}

trait KeyBoardWriter {
    fn write(&mut self, c: u8) -> bool;
    fn write(&mut self, buffer: u8, size: i8);

}

impl<'a> Keyboard<'a> {
    //fn new() -> Self {
    //    let hid = HID;
    //    let asciimap = &keyboard_layout_en_us;
    //    Self { hid, asciimap }
    //}

    fn begin(&mut self, layout: &'a [u8]) {
        self.asciimap = layout;
    }

    fn end(&mut self) {
        // Implementation for end
        // You may want to add your own logic here.
    }

    //fn send_report(&mut self, keys: &KeyReport) {
    //    self.hid.send_report(2, keys);
    //}

    // press() adds the specified key (printing, non-printing, or modifier)
    // to the persistent key report and sends the report.  Because of the way
    // USB HID works, the host acts like the key remains pressed until we
    // call release(), releaseAll(), or otherwise clear the report and resend.
    pub fn press(&mut self, mut k: u8) -> bool {
        if (k >= 136) {
            k = k - 136;
        } else if (k >= 128) {
            self.keyreport.modifiers |= (1 << (k - 128));
            k = 0;
        } else {
            k = pgm_read_byte(_asciimap + k);
            if (!k) {
                //setWriteError();
                return false;
            }
            if ((k & ALT_GR) == ALT_GR) {
                self.keyreport.modifiers |= 0x40;   // AltGr = right Alt
                k &= 0x3F;
            } else if ((k & SHIFT) == SHIFT) {
                self.keyreport.modifiers |= 0x02;    // the left shift modifier
                k &= 0x7F;
            }
            if (k == ISO_REPLACEMENT) {
                k = ISO_KEY;
            }
        }
        // Add k to the key report only if it's not already present
        // and if there is an empty slot.
        if (self.keyreport.keys[0] != k && self.keyreport.keys[1] != k &&
            self.keyreport.keys[2] != k && self.keyreport.keys[3] != k &&
            self.keyreport.keys[4] != k && self.keyreport.keys[5] != k) {
            let mut i = 0;
            while i < 6 {
                if self.keyreport.keys[i] == 0x00 {
                    self.keyreport.keys[i] = k;
                    break;
                }
                i += 1;
            }

            if i == 6 {
                //setWriteError();
                return false;
            }
        }
        //sendReport(&self.keyreport);
        return true;
    }

    fn sendReport(key_report: &KeyReport) {}

    pub fn write(&mut self, c: u8) -> bool {
        let p: bool = self.press(c);
        self.release(c);
        return p;
    }

    pub fn write(&mut self, buffer: u8, size: i8) {
        let n: i8 = 0;
    }

    pub fn releaseAll(&mut self) {
        self.keyreport[0] = 0;
        self.keyreport[1] = 0;
        self.keyreport[2] = 0;
        self.keyreport[3] = 0;
        self.keyreport[4] = 0;
        self.keyreport[5] = 0;
        self.keyreport.modifiers = 0;
        self.sendReport(&self.keyreport);
    }
}

