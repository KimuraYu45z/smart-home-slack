use regex::Regex;
use wake_on_lan::*;

pub fn mac_validate(str_mac:&str)->bool{
    let set = Regex::new(r"^([0-9A-Fa-f]{2}[:]){5}([0-9A-Fa-f]{2})$").unwrap();
    return is_match(set, str_mac);
}

pub fn convert_mac_address(str_mac:&str)->[u8;6]{
    let octet: Vec<_> = str_mac.to_String().split(":");
    return hex::decode(octet);
}

pub fn wake_on_lan(mac_address: [u8; 6]) -> Result<(), std::io::Error> {
    let magic_packet = MagicPacket::new(&mac_address);

    // Send the magic packet via UDP to the broadcast address 255.255.255.255:9 from 0.0.0.0:0
    magic_packet.send()?;

    Ok(())
}
