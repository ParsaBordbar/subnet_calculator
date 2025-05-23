// (CIDR) is an IP address allocation method. 

mod prefix;
mod subnet;
mod utils;
mod cidr;

use crate::utils::{
    input,
    wait_for_exit,
};
use crate::subnet::{
    subnets,
    subnet_allocator,
    display_subnet_allocation,
};
use crate::cidr::{
    CidrInfo,
    parse_cidr
};


fn main() {
    let input_value = input("Enter CIDR (e.g. 192.168.1.0/24):");
    let main_network = parse_cidr(&input_value);
    
    match &main_network {
        Ok(info) => {
            println!("\n  ┌── CIDR OF {}", input_value);
            println!("  ├── Network:     {}", info.network);
            println!("  ├── Broadcast:   {}", info.broadcast);
            println!("  ├── Subnet Mask: {}", info.subnet_mask);
            println!("  ├── Total Hosts: {}", info.host_count);
            if let Some(first) = info.first_host {
                println!("  ├── First Host:  {}", first);
            }
            if let Some(last) = info.last_host {
                println!("  └── Last Host:   {} \n", last);
            }
            
            let subnets_count: u32 = input("\nHow many subnets do you want?: ")
                .parse().expect("Error not able to cast to u32");

            let subnets = subnets(subnets_count);
            let allocated_subnets = subnet_allocator(subnets, info.network);
            
            display_subnet_allocation(&allocated_subnets);
            wait_for_exit();
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
