use std::net::{UdpSocket, SocketAddr, IpAddr};
use std::env;

fn collection_ip(aggressive: bool, ip: std::net::Ipv4Addr) -> Vec<std::net::SocketAddr> {
    let mut collect_targets : Vec<std::net::SocketAddr> = Vec::new();
    let aggressive_ports = vec![7, 40557, 47536, 44099, 38482, 46613];
    collect_targets.push(SocketAddr::new(IpAddr::V4(ip), 9));
    if aggressive {
        for port in aggressive_ports.iter() {
            collect_targets.push(SocketAddr::new(IpAddr::V4(ip), *port))
        }
    }
    collect_targets
}

pub struct CollectedIpTargets {
    pub socket_addr: std::net::SocketAddr,
    pub mac: String
}

fn mac_address_correct(mac: String) -> Result<String, bool> {
    if mac.len() != 17 {
        Err(false)
    } else if mac.split(":").collect::<String>().len() != 12 {
        Err(false)
    } else {
        Ok(mac.split("-").collect())
    }
}

pub fn listen_packages(ip: String) {
    let com =  UdpSocket::bind((ip, 9));
    if let Ok(socket) = com {
        println!("LISTENING");
        //socket.set_read_timeout(Some(Duration::new(5, 0)))?;
        socket.set_broadcast(true).unwrap();
        loop {
            let mut b = [0 as u8; 102];
            if let Ok((_,e)) = socket.recv_from(&mut b) {
                println!("--- {} ---",e);
                println!("{:02x?}", &b[..]);
                println!("---END OF DATA---");
            }
        }
    } else if let Err(e) = com {
        println!("Failed to open connection");
        println!("{:#?}", e);
    }
}

pub fn collect_ip_targets(env: Vec<String>, mut aggressive: bool) -> Result<Vec<CollectedIpTargets>, String> {
    let mut collected_addresses : Vec<CollectedIpTargets> = Vec::new();
    let mut collect_mac = false;
    let mut collected_mac : String = "".to_string();
    let mut collect_ip  = false;
    let mut err = false;
    let mut err_text = "".to_string();
    for key in env.iter() {
        if key.starts_with("-") {
            if key == "--ip" || key == "-i" {
                if collected_mac == "" {
                    err = true;
                    break;
                } else {
                    collect_ip = true;
                }
            } else if key == "--mac" || key == "-m" {
                collect_mac = true
            } else if key == "--aggressive" || key == "-a" {
                aggressive = true;
            }
        } else if collect_ip {
            if !collect_mac {
                err = true;
                break;
            } else {
                if let Ok(ip_address) = key.parse::<std::net::Ipv4Addr>() {
                    collect_ip = false;
                    collect_mac = false;
                    let mac_correct = mac_address_correct(collected_mac.clone());
                    if let Ok(mac) = mac_correct {
                        for address in collection_ip(aggressive, ip_address.clone()).iter() {
                            collected_addresses.push(CollectedIpTargets {
                                socket_addr: *address,
                                mac: mac.clone()
                            });
                        }
                    } else {
                        err = true;
                        err_text = collected_mac.clone()
                    }
                } else {
                    err = true;
                    err_text = key.to_string();
                    break;
                }
            }
        } else if collect_mac {
            collected_mac = key.to_string();
        }
    }
    if err {
        Err::<Vec<CollectedIpTargets>, String>(err_text)
    } else if (collect_mac || collect_ip) && collected_addresses.len() == 0 {
        Err::<Vec<CollectedIpTargets>, String>("".to_string())
    } else {
        Ok::<Vec<CollectedIpTargets>, String>(collected_addresses)
    }
}

pub fn send_package(addr: std::net::SocketAddr, mac: String) -> Result<bool,usize> {
    let com =  UdpSocket::bind("0.0.0.0:0");
    if let Ok(socket) = com {
        socket.set_broadcast(true).unwrap();
        if let Ok(magic_package) = wole::generate_magic_package(&mac) {
            let connection = socket.send_to(&magic_package, addr.clone());
            if let Ok(_) = connection {
                Ok(true)
            } else if let Err(_) = connection {
                Err(1)
            } else {
                Err(3)
            }
        } else {
            Err(2)
        }
    } else if let Err(_) = com {
        Err(0)
    } else {
        Err(3)
    }
}

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
                println!("Listen for WOL packages in common ports");
                println!("--listen | -l [192.168.1.100]");
            } else {
                println!("Arguments Help:");
                println!("  --mac        | -m : Mac address of target");
                println!("  --ip         | -i : IP address of target");
                println!("  --aggressive | -a : Aggressive packages");
                println!("  --listen     | -l : Listen for WOL packages\n\n");
                println!("  Target One Device: (--mac | -m) XX:XX:XX:XX:XX:XX (--ip | -i) 000.000.0.0");
                println!("  Target Multiple Devices: (--mac | -m) XX:XX:XX:XX:XX:XX (--ip | -i) 000.000.0.0 (--mac | -m) XX:XX:XX:XX:XX:XX (--ip | -i) 000.000.0.0");
                println!("  Listen For WOL Packages: (--listen | -l) 000.000.0.0")
            }
        } else if args.contains(&"-v".to_string()) || args.contains(&"--version".to_string()) {
            const VERSION: &'static str = env!("CARGO_PKG_VERSION");
            println!("Wake-On-Lan Magic Package Generator & Sender & CLI\nv{}",VERSION);
        } else {
            if args.contains(&"-l".to_string()) || args.contains(&"--listen".to_string()) {
                let indx = args.iter().position(|r| r == "-l" || r == "--listen").unwrap();
                if let Some(ip) = args.get(indx + 1) {
                    listen_packages(ip.to_string());
                } else {
                    println!("Wrong usage of args");
                    println!("  Type --help for usage")
                }
            } else {
                let collected = collect_ip_targets(args.clone(), args.contains(&"--agressive".to_owned()) || args.contains(&"-a".to_owned()));
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
                        let sent = send_package(address.socket_addr, address.mac.clone());
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
