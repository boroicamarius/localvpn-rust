pub struct TcpPacket {
    pub uninitialized: bool,
    pub source_port: u16,
    pub dest_port: u16,
    pub seq_num: u32,
    pub ack_num: u32,
    pub ack: bool,
    pub rst: bool,
    pub syn: bool,
    pub fin: bool,
    pub window_size: u16,
    pub payload: Vec<u8>,
}

impl TcpPacket {
    pub fn default() -> Self {
        Self {
            uninitialized: true,
            source_port: 0,
            dest_port: 0,
            seq_num: 0,
            ack_num: 0,
            ack: false,
            rst: false,
            syn: false,
            fin: false,
            window_size: 0,
            payload: vec![],
        }
    }
    pub fn new_raw(mut data: Vec<u8>) -> Self {
        let mut tcp_packet: TcpPacket = TcpPacket::default();
        tcp_packet.source_port = ((data[0] as u16).checked_shl(8).unwrap()) + (data[1] as u16);
        tcp_packet.dest_port = ((data[2] as u16).checked_shl(8).unwrap()) + (data[3] as u16);
        tcp_packet.ack_num = ((data[4] as u32).checked_shl(24).unwrap())
            + ((data[5] as u32).checked_shl(16).unwrap())
            + ((data[6] as u32).checked_shl(8).unwrap())
            + (data[7] as u32);
        tcp_packet.seq_num = ((data[8] as u32).checked_shl(24).unwrap())
            + ((data[9] as u32).checked_shl(16).unwrap())
            + ((data[10] as u32).checked_shl(8).unwrap())
            + (data[11] as u32);
        let data_offset: u16 = (data[12] as u16).checked_shr(4).unwrap() * 4;
        tcp_packet.payload = data
            .drain((data_offset as usize)..((data.len()) as usize))
            .collect();
        tcp_packet.ack = data[13].checked_shr(4).unwrap().checked_rem(2).unwrap() != 0;
        tcp_packet.rst = data[13].checked_shr(2).unwrap().checked_rem(2).unwrap() != 0;
        tcp_packet.syn = data[13].checked_shr(1).unwrap().checked_rem(2).unwrap() != 0;
        tcp_packet.fin = data[13].checked_rem(2).unwrap() != 0;
        tcp_packet.uninitialized = false;
        tcp_packet
    }
}
