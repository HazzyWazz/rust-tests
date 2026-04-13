#![allow(non_snake_case)]
fn getBinaryRep(address: &str) -> String {
    let ipvec: Vec<String> = address.split(".")
    .map(|octetstr| octetstr.parse::<u8>().unwrap())
    .map(|octet| format!("{:08b}", octet)).collect();
    ipvec.join(".")
}

fn longMaskToBinRep(longmask: &str) -> String {
    longmask.chars()
    .collect::<Vec<char>>()
    .chunks(8)
    .map(|c| c.iter().collect::<String>())
    .collect::<Vec<String>>()
    .join(".")
}

fn maskFromPrefix(prefix: u8) -> String {
    let hosts = "0".repeat((32-prefix).to_string().parse::<usize>().unwrap());
    let prefixbytes = "1".repeat(prefix.to_string().parse::<usize>().unwrap());
    let longmask = prefixbytes + &remainder;
    longMaskToBinRep(&longmask)
    
    // let intval = isize::from_str_radix(prefix, 2).unwrap();  may be useful later
}

fn main() {
    //hosts = 2^h - 2, where h = trailing zeros of mask
    let ip = "172.16.1.32";
    let mask = "255.255.0.0";
    
    let binip = getBinaryRep(ip);
    let binmask = getBinaryRep(mask);
    
    println!("{:?}", binip);
    println!("{:?}", binmask);
    
    println!("{:?}", maskFromPrefix(16));  
}