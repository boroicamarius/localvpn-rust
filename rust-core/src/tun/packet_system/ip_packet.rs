#[path = "ipv4packet.rs"]
pub mod ipv4packet;
#[path = "ipv6packet.rs"]
pub mod ipv6packet;
use ipv4packet::Ipv4Packet;
use ipv6packet::Ipv6Packet;
pub enum PacketType {
    Ipv4(ipv4packet::Ipv4Packet),
    Ipv6(ipv6packet::Ipv6Packet),
    Uninitialized,
}
pub enum PacketVersion {
    Ipv4,
    Ipv6,
    Uninitialized,
}

pub struct IpPacket {
    packet: PacketType,
}

impl IpPacket {
    fn default() -> Self {
        Self {
            packet: PacketType::Uninitialized,
        }
    }
    pub fn create(version: PacketVersion, data: Vec<u8>) -> Self {
        match version {
            PacketVersion::Ipv4 => Self {
                packet: PacketType::Ipv4(match Ipv4Packet::new_raw(data) {
                    Ok(pkt) => pkt,
                    Err(e) => {
                        log::trace!("PACKET ERR: {}", e);
                        Ipv4Packet::default()
                    }
                }),
            },
            PacketVersion::Ipv6 => Self {
                // packet: PacketType::Ipv6(Ipv6Packet::new(data)),
                ..IpPacket::default()
            },
            PacketVersion::Uninitialized => IpPacket::default(),
        }
    }
}
