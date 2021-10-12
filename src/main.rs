// extern crate reqwest;




// pub mod slack;
pub mod wol;

fn main() {
    let mac: &str="18:C0:4D:94:06:7F";
    println!("Mac Validity : ",mac_validate(mac));
    if mac_validate(mac){
        println!("MacAddress : ",convert_mac_address(mac));
    }
    
}
