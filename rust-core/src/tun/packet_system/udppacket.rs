pub struct UdpPacket {
    pub uninitialized: bool,
    pub source_port: u16,
    pub dest_port: u16,
    pub length: u16,
    pub payload: Vec<u8>,
}

impl UdpPacket {
    pub fn default() -> Self {
        Self {
            uninitialized: true,
            source_port: 0,
            dest_port: 0,
            length: 0,
            payload: vec![],
        }
    }
    pub fn new_raw(mut data: Vec<u8>) -> Self {
        let mut udp_packet = UdpPacket::default();
        udp_packet.source_port = ((data[0] as u16).checked_shl(8).unwrap()) + (data[1] as u16);
        udp_packet.dest_port = ((data[2] as u16).checked_shl(8).unwrap()) + (data[3] as u16);
        udp_packet.length = ((data[4] as u16).checked_shl(8).unwrap()) + (data[5] as u16);
        udp_packet.payload = data.drain((8 as usize)..(data.len() as usize)).collect();
        udp_packet.uninitialized = false;
        udp_packet
    }
}
