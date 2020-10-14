use lib;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        println!("Wake-On-Lan CLI");
        println!("  type -h or --help for help")
    } else {
        if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
            if args.contains(&"-m".to_string()) || args.contains(&"--mac".to_string()) {
                println!("[REQUIRED] Mac address of target");
                println!("--mac | -m [2C:2C:2C:2C:2C:2C]");
            } else if args.contains(&"-i".to_string()) || args.contains(&"--ip".to_string()) {
                println!("[REQUIRED] IP address of target");
                println!("--ip | -i [192.168.1.100]");
            } else if args.contains(&"-m".to_string()) || args.contains(&"--multiple".to_string()) {
                println!("Target Multiple Devices");
                println!("If key supplied --mac & --ip args must be applied one after another");
            } else if args.contains(&"-a".to_string()) || args.contains(&"--aggressive".to_string()) {
                println!("Send packages aggressively to following ports one after another");
                println!("[9, 7, 40557, 47536, 44099, 38482, 46613]");
            } else if args.contains(&"-l".to_string()) || args.contains(&"--listen".to_string()) {
                println!("Listen for WOL packages in common ports")
            } else {
                println!("Arguments Help:");
                println!("  --mac        | -m : Mac address of target");
                println!("  --ip         | -i : IP address of target");
                println!("  --aggressive | -a : Aggressive packages");
                println!("  --listen     | -l : Listen for WOL packages\n\n");
                println!("  Target One Device: (--mac | -m) XX:XX:XX:XX:XX:XX (--ip | -i) 000.000.0.0");
                println!("  Target Multiple Devices: (--mac | -m) XX:XX:XX:XX:XX:XX (--ip | -i) 000.000.0.0 (--mac | -m) XX:XX:XX:XX:XX:XX (--ip | -i) 000.000.0.0");
                println!("  Listen For WOL Packages: (--listen | -l)")
            }
        } else {
            if args.contains(&"-l".to_string()) || args.contains(&"--listen".to_string()) {
                println!("Listening Packages Not Implemented Yet");
            } else {
                let collected = lib::collect_ip_targets(args.clone(), args.contains(&"--agressive".to_owned()) || args.contains(&"-a".to_owned()));
                if let Err(e) = collected {
                    if e == "" {
                        println!("Wrong usage of args");
                        println!("  Type --help for usage")
                    } else {
                        println!("Wrong usage of args, {} is not correct value", e);
                        println!("  Type --help for usage")
                    }
                } else if let Ok(addresses) = collected {
                    for address in addresses.iter() {
                        let sent = lib::send_package(address.socket_addr, address.mac.clone());
                        if let Err(code) = sent {
                            if code == 0 {
                                println!("[ERR] Failed to connect device {}:{}", address.socket_addr.ip(), address.socket_addr.port());
                            } else if code == 1 {
                                println!("[ERR] Failed to send packages  {}:{}", address.socket_addr.ip(), address.socket_addr.port());
                            } else if code == 2 {
                                println!("[ERR] Failed to generate magic package [Wrong Mac Address] {}:{}", address.socket_addr.ip(), address.socket_addr.port());
                            } else {
                                println!("[ERR] Unknown error            {}:{}", address.socket_addr.ip(), address.socket_addr.port());
                            }
                        } else {
                            println!("[OK]  Package Sent {}:{}", address.socket_addr.ip(), address.socket_addr.port());
                        }
                    }
                }
            }
        }
    }
}
