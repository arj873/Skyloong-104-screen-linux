use rusb::{
    ConfigDescriptor, DeviceDescriptor, DeviceHandle, DeviceList, EndpointDescriptor,
    InterfaceDescriptor, Language, Result, Speed, UsbContext,
};
use std::time::Duration;

use usb_ids::{self, FromId};

struct UsbDevice<T: UsbContext> {
    handle: DeviceHandle<T>,
    language: Language,
    timeout: Duration,
}

fn main() {
    list_devices().unwrap();
}

fn list_devices() -> Result<()> {
    let timeout = Duration::from_secs(1);

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
