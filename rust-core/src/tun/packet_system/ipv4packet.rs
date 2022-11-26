pub struct Ipv4Packet {
    pub uninitialized: bool,
    pub total_length: u16,
    pub protocol: u8,
    pub source_ip: [u8; 4],
    pub dest_ip: [u8; 4],
    pub payload: Vec<u8>,
}

impl Ipv4Packet {
    pub fn default() -> Self {
        Self {
            uninitialized: true,
            total_length: 0,
            protocol: 0,
            source_ip: [0, 0, 0, 0],
            dest_ip: [0, 0, 0, 0],
            payload: vec![],
        }
    }
    pub fn new_raw(mut data: Vec<u8>) -> Self {
        if data.len() < 5 {
            return Ipv4Packet::default();
        }
        let mut new_packet: Self = Ipv4Packet::default();
        let mut ihl = data[0].checked_rem(16).unwrap();
        new_packet.total_length = ((data[2] as u16).checked_shl(8).unwrap()) + (data[3] as u16);
        new_packet.protocol = data[9];
        new_packet.source_ip[0] = data[12];
        new_packet.source_ip[1] = data[13];
        new_packet.source_ip[2] = data[14];
        new_packet.source_ip[3] = data[15];
        new_packet.dest_ip[0] = data[16];
        new_packet.dest_ip[1] = data[17];
        new_packet.dest_ip[2] = data[18];
        new_packet.dest_ip[3] = data[19];

        let index = 20 + if ihl > 5 { (ihl - 5) * 4 } else { 0 };
        new_packet.payload = data
            .drain((index as usize)..(new_packet.total_length as usize))
            .collect();
        new_packet.uninitialized = false;
        new_packet
    }
}
