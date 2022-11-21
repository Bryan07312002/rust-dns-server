#[path = "../byte_packet.rs"]
mod byte_packet;
use crate::dns_packet::BytePacketBuffer;

#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
pub enum QueryType {
    UNKNOWN(u16),
    A, // 1
}

impl QueryType {
    pub fn from_num(num: u16) -> QueryType {
        match num {
            1 => QueryType::A,
            _ => QueryType::UNKNOWN(num)
        }
    }

    pub fn to_enum(&self) -> u16 {
        match *self {
            QueryType::UNKNOWN(x) => x,
            QueryType::A => 1
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DnsQuestion {
    pub name: String,
    pub qtype: QueryType
}

impl DnsQuestion {
    pub fn new(name:String, qtype:QueryType) -> DnsQuestion {
        DnsQuestion {
            name: name,
            qtype: qtype
        }
    }

    pub fn read(&mut self, buffer:&mut BytePacketBuffer) -> Result<(), ()> {
        buffer.read_qname(&mut self.name)?;
        self.qtype = QueryType::from_num(buffer.read_u16()?);
        Ok(())
    }
}