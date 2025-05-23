use std::net::Ipv4Addr;

// pub fn hosts_from_prefix(prefix: u8) -> u32 {
//     match prefix {
//         31 => 2,
//         32 => 1,
//         _ if prefix < 31 => 2u32.pow((32 - prefix) as u32) - 2, // Normal case: 2^(32-prefix) - 2
//         _ => 0,
//     }
// }

pub fn prefix_from_hosts(hosts: u32) -> u8 {
    match hosts {
        0 => 32,
        1 => 32,
        2 => 30,
        _ => {
            let needed_bits = (hosts as f64 + 2.0).log2().ceil() as u32;
            if needed_bits >= 32 {
                0
            } else {
                (32 - needed_bits) as u8
            }
        }
    }
}

pub fn prefix_to_subnet_mask(prefix: u8) -> Ipv4Addr {
    let mask = if prefix == 0 {
        0
    } else {
        (!0u32) << (32 - prefix)
    };

    Ipv4Addr::from(mask)
}