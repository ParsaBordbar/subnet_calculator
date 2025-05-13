![image](https://github.com/user-attachments/assets/bc8126ac-f2f9-48e8-871c-1b2933278a95)

# SUB-NET CALCULATOR

## Overview
I was'nt so good at allocating subnets So, I write it with rust!
What is this this tool exactly? 
This Rust-based CIDR (Classless Inter-Domain Routing) Subnet Calculator is a command-line tool designed to help network administrators and IT professionals efficiently plan and allocate IP network subnets.

## Features

- Parse and analyze existing CIDR networks
- Calculate subnet masks dynamically
- Automatically determine optimal prefix lengths
- Support for multiple subnet allocations
- Detailed subnet information output

## Prerequisites

- Rust Programming Language (latest stable version recommended)
- Cargo (Rust's package manager)

## Installation

### Run it Via Binnary 

#### On Linux

just run the binnary
```bash
    ./subnet
```

### Clone the Repository

```bash
git clone https://github.com/yourusername/cidr-subnet-calculator.git
cd cidr-subnet-calculator
```

### Build the Project

```bash
cargo build --release
```

### Run the Application

```bash
cargo run
```

## Key Concepts

- **CIDR Notation**: Represents IP networks using a compact format (`IP_ADDRESS/PREFIX`)
- **Prefix Length**: Determines the size of the network (smaller prefix = larger network)
- **Host Count**: The number of usable IP addresses in a subnet

## Planned Features

- [ ] Terminal User Interface (TUI) support
- [ ] More advanced network planning features

## License

Distributed under the MIT License. See `LICENSE` for more information.

---
