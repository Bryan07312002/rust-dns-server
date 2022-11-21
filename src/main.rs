#[path = "./dns_packet/dns_packet.rs"]
mod dns_packet;
mod byte_packet;
use crate::byte_packet::BytePacketBuffer;
use crate::dns_packet::DnsPacket;
use std::fs::File;
use std::io::Read;

#[warn(dead_code)]
fn main() -> Result<(), ()> {
    let mut f = File::open("response_packet.txt").ok().expect("opens");
    let mut buffer = BytePacketBuffer::new();
    f.read(&mut buffer.buf).ok().expect("reads");

    let packet = DnsPacket::from_buffer(&mut buffer)?;
    println!("{:#?}", packet.header);

    for q in packet.questions {
        println!("{:#?}", q);
    }
    for rec in packet.answers {
        println!("{:#?}", rec);
    }
    for rec in packet.authorities {
        println!("{:#?}", rec);
    }
    for rec in packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}
