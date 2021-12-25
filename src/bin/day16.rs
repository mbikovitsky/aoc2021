use anyhow::{bail, Result};
use itertools::Itertools;

use aoc2021::{
    bitmap::{BitMap, BitMapRef},
    util::input_lines,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Packet {
    version: u8,
    data: PacketData,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum PacketData {
    Literal(u64),
    Operator(Vec<Packet>),
}

impl Packet {
    fn parse<'a>(data: &BitMapRef<'a>) -> (Self, usize) {
        let version: u64 = data.slice(0..3).try_into().unwrap();
        let version: u8 = version.try_into().unwrap();

        let type_id: u64 = data.slice(3..6).try_into().unwrap();
        let (data, len) = match type_id {
            4 => {
                let mut literal = 0u64;
                let mut len = 0usize;
                for index in 0.. {
                    let bits = data.slice(6 + index * 5..6 + (index + 1) * 5);

                    let current: u64 = bits.slice(1..5).try_into().unwrap();
                    literal <<= 4;
                    literal |= current;

                    len += 5;

                    if !bits.get(0) {
                        break;
                    }
                }
                (PacketData::Literal(literal), len)
            }
            _ => {
                let length_type_id = data.get(6);

                if length_type_id {
                    let num_packets: u64 = data.slice(7..18).try_into().unwrap();
                    let num_packets: usize = num_packets.try_into().unwrap();

                    let mut packets = Vec::with_capacity(num_packets);
                    let mut len = 0;
                    for _ in 0..num_packets {
                        let (packet, packet_len) = Self::parse(&data.slice(18 + len..data.len()));
                        packets.push(packet);
                        len += packet_len;
                    }

                    (PacketData::Operator(packets), len + 12)
                } else {
                    let len: u64 = data.slice(7..22).try_into().unwrap();
                    let len: usize = len.try_into().unwrap();

                    let mut packets = Vec::new();
                    let mut read = 0;
                    while read < len {
                        let (packet, packet_len) = Self::parse(&data.slice(22 + read..22 + len));
                        packets.push(packet);
                        read += packet_len;
                    }

                    (PacketData::Operator(packets), len + 16)
                }
            }
        };

        (Self { version, data }, len + 6)
    }
}

impl From<&[u8]> for Packet {
    fn from(bytes: &[u8]) -> Self {
        let bitmap = BitMap::new(Vec::from(bytes));
        let bitmap = bitmap.slice(0..bitmap.len());
        Self::from(&bitmap)
    }
}

impl<'a> From<&BitMapRef<'a>> for Packet {
    fn from(data: &BitMapRef<'a>) -> Self {
        Self::parse(data).0
    }
}

fn main() -> Result<()> {
    let bytes = parse_input()?;
    let packet = Packet::from(&bytes[..]);

    let version_sum = sum_all_version_numbers(&packet);
    dbg!(version_sum);

    Ok(())
}

fn sum_all_version_numbers(packet: &Packet) -> u32 {
    let mut sum = packet.version.into();

    if let PacketData::Operator(packets) = &packet.data {
        for packet in packets {
            sum += sum_all_version_numbers(packet);
        }
    }

    sum
}

fn parse_input() -> Result<Vec<u8>> {
    let line = if let Ok(line) = input_lines()?.exactly_one() {
        line?
    } else {
        bail!("Invalid number of lines in input file");
    };

    if !line.is_ascii() {
        bail!("Invalid characters in input");
    }

    if !line.len() % 2 == 0 {
        bail!("Invalid count of characters in input");
    }

    let mut bytes = Vec::with_capacity(line.len() / 2);

    for index in 0..line.len() / 2 {
        let byte = u8::from_str_radix(&line[index * 2..index * 2 + 2], 16)?;
        bytes.push(byte);
    }

    Ok(bytes)
}
