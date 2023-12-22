use core::iter::Iterator;
use core::todo;

use crate::keyboard_layout::*;

fn set_write_error() {
    todo!("HDI?")
}

pub struct KeyReport {
    modifiers: u8,
    reserved: u8,
    keys: [u8; 6],
}

impl KeyReport {
    fn send_report(&self) {
        todo!("HDI?")
    }
}

pub enum KeyMap {
    German,
}

impl KeyMap {
    fn get_map(&self) -> [u8; 128] {
        match self {
            KeyMap::German => crate::keyboard_layout_de_de::keyboard_layout_de_de,
        }
    }

    fn get_at_offset(&self, offset: u8) -> u8 {
        self.get_map()[offset as usize]
    }
}

pub struct Keyboard {
    key_report: KeyReport,
    ascii_map: KeyMap,
}

impl Keyboard {
    pub fn new(key_report: KeyReport, ascii_map: KeyMap) -> Self {
        todo!("HDI?");

        Keyboard {
            key_report,
            ascii_map,
        }
    }

    pub fn end(&self) {}

    pub fn write_single(&mut self, c: u8) -> bool {
        let p = self.press(c);

        self.release(c);

        p
    }

    pub fn write(&mut self, values: &[u8]) -> usize {
        let mut n = 0;

        for item in values {
            if *item != b'\r' {
                if self.write_single(*item) {
                    n += 1;
                } else {
                    break;
                }
            }
        }

        return n;
    }

    pub fn press(&mut self, mut k: u8) -> bool {
        if k >= 136 {
            k = k - 136;
        } else if k >= 128 {
            self.key_report.modifiers |= 1 << (k - 128);
            k = 0;
        } else {
            k = self.ascii_map.get_at_offset(k);

            if k == 0 {
                set_write_error();
                return false;
            }

            if (k & ALT_GR) == ALT_GR {
                self.key_report.modifiers |= 0x40;
                k &= 0x3F;
            } else if (k & SHIFT) == SHIFT {
                self.key_report.modifiers |= 0x02;
                k &= 0x7F;
            }

            if k == ISO_REPLACEMENT {
                k = ISO_KEY;
            }
        }

        // TODO check assembly for unrolled 6x AND's
        if self
            .key_report
            .keys
            .iter()
            .map(|item| *item != k)
            .fold(true, |acc, item| acc && item)
        {
            let mut i = 0;
            while i < 6 {
                if self.key_report.keys[i] == 0x00 {
                    self.key_report.keys[i] = k;
                    break;
                }

                i += 1;
            }

            if i == 6 {
                set_write_error();
                return false;
            }
        }

        self.key_report.send_report();

        true
    }

    pub fn release(&mut self, mut k: u8) -> bool {
        if k >= 136 {
            k = k - 136;
        } else if k >= 128 {
            self.key_report.modifiers &= !(1 << (k - 128));
            k = 0;
        } else {
            k = self.ascii_map.get_at_offset(k);

            if k == 0 {
                return false;
            }

            if (k & ALT_GR) == ALT_GR {
                self.key_report.modifiers &= !0x40;
                k &= 0x3F;
            } else if (k & SHIFT) == SHIFT {
                self.key_report.modifiers &= !0x02;
                k &= 0x7F;
            }

            if k == ISO_REPLACEMENT {
                k = ISO_KEY;
            }
        }

        for i in 0..6 {
            if self.key_report.keys[i] == 0x00 {
                self.key_report.keys[i] = k;
                break;
            }
        }

        self.key_report.send_report();

        true
    }

    pub fn release_all(&mut self) {
        self.key_report.keys = [0; 6];
        self.key_report.modifiers = 0;

        self.key_report.send_report();
    }
}
