extern crate hex;

use std::convert::TryInto;

use hex::*;
use regex::Regex;
use wake_on_lan::*;

fn validate_mac_address(str_mac: &str) -> bool {
    let set = Regex::new(r"^([0-9A-Fa-f]{2}[:]){5}([0-9A-Fa-f]{2})$").unwrap();
    return Regex::is_match(&set, str_mac);
}

pub fn convert_mac_address(str_mac: &str) -> Result<[u8; 6], ()> {
    if !validate_mac_address(str_mac) {
        return Err(());
    }
    let macaddress = decode(str_mac.replace(":", "")).unwrap();

    Ok(macaddress.try_into().unwrap())
}

pub fn wake_on_lan(mac_address: [u8; 6]) -> Result<(), std::io::Error> {
    let magic_packet = MagicPacket::new(&mac_address);

    // Send the magic packet via UDP to the broadcast address 255.255.255.255:9 from 0.0.0.0:0
    magic_packet.send()?;

    Ok(())
}
