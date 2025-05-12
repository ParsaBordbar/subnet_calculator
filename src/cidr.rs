use std::net::Ipv4Addr;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub struct CidrInfo {
    pub network:      Ipv4Addr,
    pub broadcast:    Ipv4Addr,
    pub first_host:   Option<Ipv4Addr>,
    pub last_host:    Option<Ipv4Addr>,
    pub host_count:   u32,
    pub subnet_mask:  Ipv4Addr,
    pub cidr_notation: String,
}


pub fn parse_cidr(cidr: &str) -> Result<CidrInfo, String> {
    let parts: Vec<&str> = cidr.split('/').collect();
    if parts.len() != 2 {
        return Err("CIDR must be in format x.x.x.x/prefix".into());
    }

    let ip_str = parts[0];
    let prefix: u8 = parts[1]
        .parse()
        .map_err(|e: ParseIntError| format!("Invalid prefix: {}", e))?;
    if prefix > 32 {
        return Err("Prefix must be between 0 and 32".into());
    }

    let ip = Ipv4Addr::from_str(ip_str)
        .map_err(|_| format!("Invalid IPv4 address: {}", ip_str))?;
    let ip_u32 = u32::from(ip);

    let mask_u32 = if prefix == 0 {
        0
    } else {
        (!0u32) << (32 - prefix)
    };

    let network_u32   = ip_u32 & mask_u32;
    let broadcast_u32 = network_u32 | (!mask_u32);

    let (first, last, count) = match prefix {
        32 => (None, None, 1),
        31 => (Some(network_u32), Some(broadcast_u32), 2),
        _  => {
            let first = network_u32 + 1;
            let last  = broadcast_u32 - 1;
            let count = broadcast_u32 - network_u32 - 1;
            (Some(first), Some(last), count)
        }
    };

    let first_ip  = first.map(Ipv4Addr::from);
    let last_ip   = last.map(Ipv4Addr::from);
    let subnet_ip = Ipv4Addr::from(mask_u32);
    let cidr_notation = format!("{}/{}", Ipv4Addr::from(network_u32), prefix);

    Ok(CidrInfo {
        network:    Ipv4Addr::from(network_u32),
        broadcast:  Ipv4Addr::from(broadcast_u32),
        first_host: first_ip,
        last_host:  last_ip,
        host_count: count,
        subnet_mask: subnet_ip,
        cidr_notation,
    })
}