// extern crate reqwest;
extern crate wake_on_lan;

// pub mod slack;
pub mod wol;

fn main() {
    let mac: &str="18:C0:4D:94:06:7F";
    let validity=wol::mac_validate(mac);
    println!("Mac Validity : {}",validity);
    if wol::mac_validate(mac){
        let mac_byte=wol::convert_mac_address(mac);
        println!("MacAddress : {:?}",mac_byte);
    }
    
}
