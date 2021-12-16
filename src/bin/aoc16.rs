use std::{fs, collections::VecDeque, slice::SliceIndex};

struct Bits{
    bits: VecDeque<bool>
}

impl Bits {
    fn new( input: &str ) -> Bits {
        let mut bits: VecDeque<bool> = VecDeque::new();
        for c in input.chars() {
            let value = u8::from_str_radix(&c.to_string(), 16).unwrap();
            bits.push_back(value&0x08!=0);
            bits.push_back(value&0x04!=0);
            bits.push_back(value&0x02!=0);
            bits.push_back(value&0x01!=0);
        }
        Bits{ bits }
    }

    fn get_bits( &mut self, num_bits: u32 ) -> Option<u32> {
        if num_bits <= self.bits.len() as u32 {
            let mut ret = 0;
            for _k in 0..num_bits {
                ret <<= 1;
                if self.bits.pop_front().unwrap(){
                    ret |= 0x01;    
                }
            }
            return Some(ret);
        } else {
            return None;
        }
    }
    fn bits_remaining(&self) -> u32 {
        return self.bits.len() as u32;
    }
}

fn read_packet( bits: &mut Bits, version_sum: &mut u32 ) -> (u32,u64) {
    let bits_at_start = bits.bits_remaining();
    let version = bits.get_bits(3).unwrap();
    let type_id = bits.get_bits(3).unwrap();
    *version_sum += version;
    let mut return_value: u64 = 0;
    match type_id {
        4 => {      // data
            let mut value: u64 = 0;
            loop {
                let last_marker = bits.get_bits(1).unwrap();
                for _bit in 0..4 {
                    value <<= 1;
                    value |= bits.get_bits(1).unwrap() as u64;
                }
                if last_marker == 0 { 
                    break;
                }
            }
            println!("Value {}", value);
            return_value = value as u64;
        },
        _ => {  //an operator
            println!("operator");
            let mut values: Vec<u64> = Vec::new();
            match bits.get_bits(1).unwrap() {
                0 => {
                    // next 15 bits indicate total length of sub packets
                    let len = bits.get_bits(15).unwrap();
                    let mut bits_read = 0;
                    while bits_read != len{
                        let (num_bits,value)= read_packet(bits, version_sum);
                        bits_read += num_bits;
                        values.push(value);
                    }
                },
                _ => {
                    // next 11 bits inidcate total number of sub packets
                    let num_packets = bits.get_bits(11).unwrap();
                    for _k in 0..num_packets{
                        let (_,value) = read_packet(bits, version_sum);
                        values.push(value);
                    }
                }
            }
            match type_id {
                0 => { return_value = values.iter().sum();}
                1 => { return_value = values.iter().copied().reduce( |acc,x| { acc * x } ).unwrap(); }
                2 => { return_value = *values.iter().min().unwrap(); }
                3 => { return_value = *values.iter().max().unwrap(); }
                5 => { return_value = if values[0] > values[1] { 1 } else { 0 } }
                6 => { return_value = if values[0] < values[1] { 1 } else { 0 } }
                7 => { return_value = if values[0] == values[1] { 1 } else { 0 } }

                _ => { panic!("Unknown type_id"); }
            }
        }
    }
    return ( bits_at_start - bits.bits_remaining(), return_value );
}

fn main() {
    let input = fs::read_to_string("inputs/aoc16.txt").unwrap();
    let mut bits = Bits::new(&input);
    let mut version_sum: u32 = 0;
    let (_, value) = read_packet( &mut &mut bits, &mut version_sum);
    println!("Version sum {}", version_sum);
    println!("value  {}", value);

}