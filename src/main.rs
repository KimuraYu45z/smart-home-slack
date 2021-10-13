extern crate reqwest;
extern crate wake_on_lan;

// pub mod slack;
pub mod wol;

fn main() {
    let mac: &str = "18:C0:4D:94:06:7F";

    match wol::convert_mac_address(mac) {
        Ok(mac_address) => {
            println!("Mac address : {:?}", mac_address);
        }
        Err(_) => {
            println!("Invalid mac address.");
        }
    }
}
