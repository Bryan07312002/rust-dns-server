mod dns_header;
mod dns_record;
mod dns_question;
use crate::dns_packet::dns_header::DnsHeader;
use crate::dns_packet::dns_question::DnsQuestion;
use crate::dns_packet::dns_record::DnsRecord;
use crate::byte_packet::BytePacketBuffer;
use crate::dns_packet::dns_question::QueryType;

pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authorities: Vec<DnsRecord>,
    pub resources: Vec<DnsRecord>,
}

impl DnsPacket {
    pub fn new() -> DnsPacket{
        DnsPacket {
            header: DnsHeader::new(),
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            resources: Vec::new()
        }
    }

    pub fn from_buffer(buffer:&mut BytePacketBuffer) -> Result<DnsPacket, ()> {
        let mut result = DnsPacket::new();
        result.header.read(buffer)?;
        for _ in 0..result.header.questions {
            let mut question = DnsQuestion::new("".to_string(), QueryType::UNKNOWN(0));
            question.read(buffer)?;
            result.questions.push(question);
        }

        for _ in 0..result.header.answers {
            let rec = DnsRecord::read(buffer)?;
            result.answers.push(rec);
        }

        for _ in 0..result.header.authoritative_entries {
            let rec = DnsRecord::read(buffer)?;
            result.authorities.push(rec);
        }

        for _ in 0..result.header.resource_entries {
            let rec = DnsRecord::read(buffer)?;
            result.resources.push(rec);
        }

        Ok(result)
    }
}