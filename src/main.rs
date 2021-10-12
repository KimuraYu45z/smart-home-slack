// extern crate reqwest;
extern crate wake_on_lan;

// pub mod slack;
pub mod wol;

fn main() {
    let mac: &str="18:C0:4D:94:06:7F";
    println!("Mac Validity : ",wol::mac_validate(mac));
    if mac_validate(mac){
        println!("MacAddress : ",wol::convert_mac_address(mac));
    }
    
}
