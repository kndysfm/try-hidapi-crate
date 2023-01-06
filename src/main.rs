extern crate hidapi;

use hidapi::{HidApi, HidDevice};

/// from "https://bitbucket.org/unessa/dualshock4-rust/src/master/src/dualshock4/mod.rs"
const DUALSHOCK4_VENDOR_ID:u16 = 0x54c;
// Dualshock4 product ID changed after playstation update 5.50
const DUALSHOCK4_PRODUCT_ID_NEW:u16 = 0x9cc;
const DUALSHOCK4_PRODUCT_ID_OLD:u16 = 0x5c4;

fn start(hid: HidDevice) {
    let mut buf = [0u8; 78];
    let timeout =  1000;
    
    loop {
        match hid.read_timeout(&mut buf, timeout) {
            Ok(sz) => {
                println!("Read ({} bytes): {:?}", sz, &buf[..sz]);
            },
            Err(_) => {

            }
        }
    }
}

fn main() {
    match HidApi::new() {
        Ok(api) => {
            for info in api.device_list() {
                if info.vendor_id() == DUALSHOCK4_VENDOR_ID && (
                    info.product_id() == DUALSHOCK4_PRODUCT_ID_NEW ||
                    info.product_id() == DUALSHOCK4_PRODUCT_ID_OLD ) {
                    println!("DualShock4 was found");
                    if let Ok(hid) = info.open_device(&api) {
                        println!("Succeeded to open DualShock4");
                        start(hid);
                        break;
                    } else {
                        println!("Failed to open DualShock4");
                        continue;
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        },
    }
    println!("DualShock4 was not found");
}
