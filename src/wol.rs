use wake_on_lan::*;

pub fn wake_on_lan(mac_address: [u8; 6]) -> Result<(), std::io::Error> {
    let magic_packet = MagicPacket::new(&mac_address);

    // Send the magic packet via UDP to the broadcast address 255.255.255.255:9 from 0.0.0.0:0
    magic_packet.send()?;

    Ok(())
}
