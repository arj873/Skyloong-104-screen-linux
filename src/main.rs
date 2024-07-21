use rusb::{DeviceList, Result };

use usb_ids::{self, FromId};
use std::env;
use std::path::Path;

use hex::encode;
use std::fs::read;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", &args[1]);
    // println!("{}", );
    if Path::new(&args[1]).exists() && &args[1].split(".").last().unwrap() == &"gif"{
        let dump = encode(read(&args[1]).unwrap());
        let mut chars = dump.chars();      

    }
    list_devices().unwrap();
}

fn list_devices() -> Result<()> {

    for device in DeviceList::new()?.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

    
        let vendor_name = match usb_ids::Vendor::from_id(device_desc.vendor_id()) {
            Some(vendor) => vendor.name(),
            None => "Unknown vendor",
        };
    
        let product_name =
            match usb_ids::Device::from_vid_pid(device_desc.vendor_id(), device_desc.product_id()) {
                Some(product) => product.name(),
                None => "Unknown product",
            };
    
        if vendor_name == "SHARKOON Technologies GmbH" && product_name == "Unknown product" {
            println!("{}", product_name)
        }

    }

    Ok(())
}
