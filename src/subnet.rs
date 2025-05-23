use std::net::Ipv4Addr;
use crate::utils::input;
use crate::cidr::CidrInfo;
use crate::prefix::{
    prefix_from_hosts,
    prefix_to_subnet_mask,
};

#[derive(Debug)]
pub struct SubnetRequest {
    requested_host_count: u32,
    requested_client_address: u32,
    prefix: u8,
}

// impl SubnetRequest {
//     pub fn new_subnet(requested_host_count: u32,
//          requested_client_address: u32, prefix: u8) -> Self {
//             Self {
//             requested_host_count,
//             requested_client_address,
//             prefix,
//         }
//     }
// }

pub fn subnets(count: u32) -> Vec<SubnetRequest> {
    let mut subnets = Vec::with_capacity(count as usize);
    for _ in 0..count {
        
        let requested_host_count: u32 = input("Enter requested host count: ")
            .parse().expect("Not a valid u32");

        let prefix = prefix_from_hosts(requested_host_count);
        
        let requested_client_address = input("Enter requested subnet address (can be any number for identification): ")
        .parse().expect("Not a valid u32");
        
        let new_subnet = SubnetRequest {
            requested_host_count,
            requested_client_address,
            prefix,
        };
        subnets.push(new_subnet);
    }
    
    subnets.sort_by_key(|s| s.prefix);
    
    println!("\nSorted subnet requests:");
    for (i, subnet) in subnets.iter().enumerate() {
        println!("Subnet {}: {} hosts prefix: /{})", i+1, subnet.requested_host_count, subnet.prefix);
    }
    
    subnets
}

pub fn subnet_allocator(requests: Vec<SubnetRequest>, initial_network: Ipv4Addr) -> Vec<CidrInfo> {
    
    let mut results = Vec::with_capacity(requests.len());
    let mut current_network_u32 = u32::from(initial_network);

    for request in requests {
        let prefix = request.prefix;
        let mask_u32 = u32::from(prefix_to_subnet_mask(prefix));

        current_network_u32 = current_network_u32 & mask_u32;
        
        let broadcast_u32 = current_network_u32 | (!mask_u32);
        
        let (first_host, last_host) = match prefix {
            32 => (None, None),
            31 => (Some(current_network_u32), Some(broadcast_u32)),
            _ => (
                Some(current_network_u32 + 1),
                Some(broadcast_u32 - 1),
            ),
        };

        let cidr_notation = format!("{}/{}", Ipv4Addr::from(current_network_u32), prefix);
        
        results.push(CidrInfo {
            network:     Ipv4Addr::from(current_network_u32),
            broadcast:   Ipv4Addr::from(broadcast_u32),
            first_host:  first_host.map(Ipv4Addr::from),
            last_host:   last_host.map(Ipv4Addr::from),
            host_count:  request.requested_host_count,
            subnet_mask: Ipv4Addr::from(mask_u32),
            cidr_notation,
        });
        current_network_u32 = broadcast_u32 + 1;
    }
    
    results
}

pub fn display_subnet_allocation(allocated_subnets: &Vec<CidrInfo>) {
    println!("\n=== Subnet Allocation Results ===");
    
    for (i, subnet) in allocated_subnets.iter().enumerate() {
        println!("\n  ┌── Subnet {}: {}", i+1, subnet.cidr_notation);
        println!("  ├── Network:     {}", subnet.network);
        println!("  ├── Broadcast:   {}", subnet.broadcast);
        println!("  ├── Subnet Mask: {}", subnet.subnet_mask);
        println!("  ├── Total Hosts: {}", subnet.host_count);
        if let Some(first) = subnet.first_host {
            println!("  ├── First Host:  {}", first);
        }
        if let Some(last) = subnet.last_host {
            println!("  └── Last Host:   {}", last);
        } else {
            println!("  └── ");
        }
    }
    println!("\n");
}