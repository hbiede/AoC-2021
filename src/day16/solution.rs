// Taken from https://github.com/mikefarquhar/advent_of_code_2021/blob/main/16/src/main.rs
// and modified by HBiede

struct Header {
    version: u8,
    packet_type: u8,
}

struct Literal {
    header: Header,
    value: i64,
}

struct Operator {
    header: Header,
    packets: Vec<usize>,
}

enum Packet {
    Literal(Literal),
    Operator(Operator),
}

struct Transmission {
    nodes: Vec<Packet>,
    root_node: usize,
}

impl Transmission {
    fn sum_version_numbers(&self) -> i64 {
        let packet = &self.nodes[self.root_node];
        self.sum_versions(packet)
    }

    fn sum_versions(&self, packet: &Packet) -> i64 {
        match packet {
            // Literal type
            Packet::Literal(literal) => literal.header.version as i64,
            // Operator type
            Packet::Operator(operator) => operator
                .packets
                .iter()
                .map(|&index| self.sum_versions(&self.nodes[index]))
                .fold(operator.header.version as i64, |accum, curr| accum + curr),
        }
    }

    fn calc_expression(&self) -> i64 {
        let packet = &self.nodes[self.root_node];
        self.calc_subexpression(packet)
    }

    fn calc_subexpression(&self, packet: &Packet) -> i64 {
        match packet {
            Packet::Literal(literal) => literal.value,
            Packet::Operator(operator) => {
                let mut iter = operator
                    .packets
                    .iter()
                    .map(|&index| self.calc_subexpression(&self.nodes[index]));

                match operator.header.packet_type {
                    0 => iter.sum(),
                    1 => iter.product(),
                    2 => iter.min().unwrap_or(0),
                    3 => iter.max().unwrap_or(0),
                    5 => {
                        let first = iter.next().unwrap();
                        let second = iter.next().unwrap();
                        if first > second { 1 } else { 0 }
                    }
                    6 => {
                        let first = iter.next().unwrap();
                        let second = iter.next().unwrap();
                        if first < second { 1 } else { 0 }
                    }
                    7 => {
                        let first = iter.next().unwrap();
                        let second = iter.next().unwrap();
                        if first == second { 1 } else { 0 }
                    }
                    _ => panic!(),
                }
            }
        }
    }
}

struct TransmissionParser {
    bytes: Vec<u8>,
    bit_pos: usize,
    nodes: Vec<Packet>,
    root_node: usize,
}

impl TransmissionParser {
    fn new(input: String) -> Self {
        let bytes = (0..input.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&input[i..i + 2], 16).unwrap())
            .collect();
        Self {
            bytes,
            bit_pos: 0,
            nodes: Vec::new(),
            root_node: 0,
        }
    }

    fn parse(mut self) -> Transmission {
        self.root_node = self.parse_packet();

        Transmission {
            nodes: self.nodes,
            root_node: self.root_node,
        }
    }

    fn parse_packet(&mut self) -> usize {
        let header = self.read_header();
        match header.packet_type {
            4 => self.read_literal(header),
            _ => self.read_operator(header),
        }
    }

    fn read_header(&mut self) -> Header {
        let version = self.read_u8(3);
        let packet_type = self.read_u8(3);
        Header {
            version,
            packet_type,
        }
    }

    fn read_literal(&mut self, header: Header) -> usize {
        let mut value = 0;

        loop {
            let has_next = self.read_u8(1);

            let chunk = self.read_u8(4) as i64;
            value = (value << 4) | chunk;

            if has_next == 0 {
                break;
            }
        }

        let literal = Literal { header, value };
        let packet = Packet::Literal(literal);
        let index = self.nodes.len();
        self.nodes.push(packet);

        index
    }

    fn read_operator(&mut self, header: Header) -> usize {
        match self.read_u8(1) {
            0 => self.read_op_by_len(header),
            1 => self.read_op_by_count(header),
            _ => panic!(),
        }
    }

    fn read_op_by_len(&mut self, header: Header) -> usize {
        let content_width = self.read_u16(15) as usize;
        let end_pos = self.bit_pos + content_width;

        let mut operator = Operator {
            header,
            packets: Vec::new(),
        };

        while self.bit_pos < end_pos {
            let child_index = self.parse_packet();
            operator.packets.push(child_index);
        }

        let packet = Packet::Operator(operator);
        let index = self.nodes.len();
        self.nodes.push(packet);

        index
    }

    fn read_op_by_count(&mut self, header: Header) -> usize {
        let num_packets = self.read_u16(11);

        let mut operator = Operator {
            header,
            packets: Vec::new(),
        };

        for _ in 0..num_packets {
            let child_index = self.parse_packet();
            operator.packets.push(child_index);
        }

        let packet = Packet::Operator(operator);
        let index = self.nodes.len();
        self.nodes.push(packet);

        index
    }

    fn read_u8(&mut self, n: usize) -> u8 {
        let mut value = 0;

        for _ in 0..n {
            let index = self.bit_pos / 8;
            let shift = 8 - (self.bit_pos % 8) - 1;

            let current_bit = (self.bytes[index] & (1 << shift)) >> shift;
            value = (value << 1) | current_bit;
            self.bit_pos += 1;
        }

        value
    }

    fn read_u16(&mut self, n: usize) -> u16 {
        let mut value = 0;

        for _ in 0..n {
            let index = self.bit_pos / 8;
            let shift = 8 - (self.bit_pos % 8) - 1;

            let current_bit = ((self.bytes[index] & (1 << shift)) >> shift) as u16;
            value = (value << 1) | current_bit;
            self.bit_pos += 1;
        }

        value
    }
}

pub fn part1(input: String) -> i64 {
    TransmissionParser::new(input).parse().sum_version_numbers()
}

pub fn part2(input: String) -> i64 {
    TransmissionParser::new(input).parse().calc_expression()
}