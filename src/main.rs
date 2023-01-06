extern crate hidapi;

use hidapi::HidApi;

fn main() {
    match HidApi::new() {
        Ok(api) => {
            let mut devs: Vec<_> = api.device_list().collect();
            devs.sort_by_key(|d| d.product_id());
            devs.sort_by_key(|d| d.vendor_id());
            for device in devs {
                println!("PID:{:04X}_VID:{:04X}&UP:{:04X}_U:{:04X}", 
                        device.vendor_id(), device.product_id(),
                        device.usage_page(), device.usage());
                if let Ok(hid) = device.open_device(&api) {
                    if let Ok(man) = hid.get_manufacturer_string() {
                        println!("  manufacturer: {}", man.unwrap());
                    } else {
                        println!("  failed to get manufacturer");
                    }
                    if let Ok(prd) = hid.get_product_string() {
                        println!("  product name: {}", prd.unwrap());
                    } else {
                        println!("  failed to get product name");
                    }
                    // try `let...else...` statement
                    let Ok(sn) = hid.get_serial_number_string() else {
                        println!("  failed to get serial number");
                        continue;
                    };
                    println!("  serial number: {}", sn.unwrap());
                } else {
                    println!("  it cannot be opened");
                    continue;
                }
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        },
    }
}
