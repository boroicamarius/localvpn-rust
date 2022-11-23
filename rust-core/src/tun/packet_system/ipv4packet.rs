pub struct Ipv4Packet {
    pub version: u8,
    pub ihl: u8,
    pub total_length: u16,
    pub protocol: u8,
    pub source_ip: [u8; 4],
    pub dest_ip: [u8; 4],
    pub payload: Vec<u8>,
}

impl Ipv4Packet {
    pub fn default() -> Self {
        Self {
            version: 0,
            ihl: 0,
            total_length: 0,
            protocol: 0,
            source_ip: [0, 0, 0, 0],
            dest_ip: [0, 0, 0, 0],
            payload: vec![],
        }
    }
    pub fn new_raw(mut data: Vec<u8>) -> Result<Self, &'static str> {
        if data.len() < 5 {
            return Err("Packet is too short for IPV4");
        }
        let mut new_packet: Self = Ipv4Packet::default();
        new_packet.version = data[0].checked_shr(4).unwrap();
        new_packet.ihl = data[0].checked_rem(16).unwrap();
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

        let index = 20
            + if new_packet.ihl > 5 {
                (new_packet.ihl - 5) * 4
            } else {
                0
            };
        new_packet.payload = data
            .drain((index as usize)..(new_packet.total_length as usize))
            .collect();
        Ok(new_packet)
    }
}
