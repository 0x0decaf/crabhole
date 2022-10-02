use std::fmt;

const MAX_PACKET_SIZE: usize = 1500;

#[derive(Debug, Default, Copy, Clone)]
enum DNSOpcode {
    QUERY,
    IQUERY,
    STATUS,
    UNASSIGNED,
    NOTIFY,
    UPDATE,

    #[default]
    UNKNOWN,
}

impl DNSOpcode {
    fn from_u8(input: u8) -> Self {
        match input {
            0 => DNSOpcode::QUERY,
            1 => DNSOpcode::IQUERY,
            2 => DNSOpcode::STATUS,
            3 => DNSOpcode::UNASSIGNED,
            4 => DNSOpcode::NOTIFY,
            5 => DNSOpcode::UPDATE,

            _ => DNSOpcode::UNKNOWN,
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
enum DNSResponseCode {
    NOERR,
    FORMAT_ERROR,
    SERV_FAIL,
    NAME_ERROR,
    NOT_IMPLEMENTED,
    REFUSED,

    #[default]
    UNKNOWN,
}

#[derive(Debug, Default, Copy, Clone)]
struct DNSHeader {
    id: u16,
    is_request: bool,
    opcode: DNSOpcode,
    authoritative: bool,
    truncated: bool,
    recursion_desired: bool,
    recursion_available: bool,
    response_code: DNSResponseCode,
    question_count: u16,
    answer_count: u16,
    nameserver_count: u16,
    additional_records_count: u16,
}

impl From<&[u8]> for DNSHeader {
    fn from(input: &[u8]) -> Self {
        // initial assertion - 96 bytes MUST be present to parse a DNS header.
        // TODO: What happens when assertion fails?
        assert_eq!(input.len() >= 96, true);

        // try unsafe stuff.
        let mut packet = DNSHeader::default();
        packet.id = (input[0] as u16) << 8 | input[1] as u16;
        packet.is_request = (input[2] >> 7) & 1 > 0; // check if 8th bit is set (move value to right 7 bits and AND with 1)

        let mut third_byte = input[2];
        third_byte <<= 1; // Move one to left to remove the query section.
        third_byte >>= 4; // Move 4 to right to get opcode.
        packet.opcode = DNSOpcode::from_u8(third_byte); // Convert the shifted number to an opcode.
        return packet;
    }
}

impl fmt::Display for DNSHeader {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        dbg!(self);
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    // Start a UDP listener for DNS on port 53. Grab any packets captured and process it into a packet.
    // Print the packet ID, and type of DNS request/response.

    let mut listener = tokio::net::UdpSocket::bind("0.0.0.0:53")
        .await
        .expect("Could not start DNS server");

    println!("Started DNS server");
    
    println!("Address \t\t count \t\t id \t\t type \t\t ");
    loop {
        let mut buffer = [0u8; MAX_PACKET_SIZE];
        let (count, remote) = listener.recv_from(&mut buffer).await.expect("Could not receive connection");
        if count == 0 {
            continue;
        }

        let packet = DNSHeader::from(&buffer[..]);
        println!("{} \t\t {} \t\t {} \t\t {:?} \t\t", remote.ip(), count, packet.id, packet.opcode);
    }
}
