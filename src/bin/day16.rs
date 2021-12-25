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
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan((Box<Packet>, Box<Packet>)),
    LessThan((Box<Packet>, Box<Packet>)),
    EqualTo((Box<Packet>, Box<Packet>)),
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
            0 => {
                let (packets, len) = Self::parse_array(&data.slice(6..data.len()));
                (PacketData::Sum(packets), len)
            }
            1 => {
                let (packets, len) = Self::parse_array(&data.slice(6..data.len()));
                (PacketData::Product(packets), len)
            }
            2 => {
                let (packets, len) = Self::parse_array(&data.slice(6..data.len()));
                (PacketData::Minimum(packets), len)
            }
            3 => {
                let (packets, len) = Self::parse_array(&data.slice(6..data.len()));
                (PacketData::Maximum(packets), len)
            }
            5 => {
                let (mut packets, len) = Self::parse_array(&data.slice(6..data.len()));

                assert_eq!(packets.len(), 2);
                let b = packets.pop().unwrap();
                let a = packets.pop().unwrap();

                (PacketData::GreaterThan((Box::new(a), Box::new(b))), len)
            }
            6 => {
                let (mut packets, len) = Self::parse_array(&data.slice(6..data.len()));

                assert_eq!(packets.len(), 2);
                let b = packets.pop().unwrap();
                let a = packets.pop().unwrap();

                (PacketData::LessThan((Box::new(a), Box::new(b))), len)
            }
            7 => {
                let (mut packets, len) = Self::parse_array(&data.slice(6..data.len()));

                assert_eq!(packets.len(), 2);
                let b = packets.pop().unwrap();
                let a = packets.pop().unwrap();

                (PacketData::EqualTo((Box::new(a), Box::new(b))), len)
            }
            _ => {
                panic!("Unexpected packet type");
            }
        };

        (Self { version, data }, len + 6)
    }

    fn parse_array<'a>(data: &BitMapRef<'a>) -> (Vec<Packet>, usize) {
        let length_type_id = data.get(0);

        if length_type_id {
            let num_packets: u64 = data.slice(1..12).try_into().unwrap();
            let num_packets: usize = num_packets.try_into().unwrap();

            let mut packets = Vec::with_capacity(num_packets);
            let mut len = 0;
            for _ in 0..num_packets {
                let (packet, packet_len) = Self::parse(&data.slice(12 + len..data.len()));
                packets.push(packet);
                len += packet_len;
            }

            (packets, len + 12)
        } else {
            let len: u64 = data.slice(1..16).try_into().unwrap();
            let len: usize = len.try_into().unwrap();

            let mut packets = Vec::new();
            let mut read = 0;
            while read < len {
                let (packet, packet_len) = Self::parse(&data.slice(16 + read..16 + len));
                packets.push(packet);
                read += packet_len;
            }

            (packets, len + 16)
        }
    }

    fn eval(&self) -> u64 {
        match &self.data {
            PacketData::Literal(literal) => *literal,
            PacketData::Sum(packets) => packets.iter().map(Self::eval).sum(),
            PacketData::Product(packets) => packets.iter().map(Self::eval).product(),
            PacketData::Minimum(packets) => packets.iter().map(Self::eval).min().unwrap(),
            PacketData::Maximum(packets) => packets.iter().map(Self::eval).max().unwrap(),
            PacketData::GreaterThan((a, b)) => {
                if a.eval() > b.eval() {
                    1
                } else {
                    0
                }
            }
            PacketData::LessThan((a, b)) => {
                if a.eval() < b.eval() {
                    1
                } else {
                    0
                }
            }
            PacketData::EqualTo((a, b)) => {
                if a.eval() == b.eval() {
                    1
                } else {
                    0
                }
            }
        }
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

    let value = packet.eval();
    dbg!(value);

    Ok(())
}

fn sum_all_version_numbers(packet: &Packet) -> u32 {
    let mut sum = packet.version.into();

    sum += match &packet.data {
        PacketData::Literal(_) => 0,
        PacketData::Sum(packets) => packets.iter().map(sum_all_version_numbers).sum(),
        PacketData::Product(packets) => packets.iter().map(sum_all_version_numbers).sum(),
        PacketData::Minimum(packets) => packets.iter().map(sum_all_version_numbers).sum(),
        PacketData::Maximum(packets) => packets.iter().map(sum_all_version_numbers).sum(),
        PacketData::GreaterThan((a, b)) => {
            sum_all_version_numbers(&*a) + sum_all_version_numbers(&*b)
        }
        PacketData::LessThan((a, b)) => sum_all_version_numbers(&*a) + sum_all_version_numbers(&*b),
        PacketData::EqualTo((a, b)) => sum_all_version_numbers(&*a) + sum_all_version_numbers(&*b),
    };

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
