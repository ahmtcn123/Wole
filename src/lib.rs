use std::net::{UdpSocket, SocketAddr, IpAddr};
use std::str;

pub fn generate_magic_package(mac: &str) -> Result<[u8; 102], bool> {
    let mut err = false;
    let mut package :[u8; 102] = [0; 102];
    for i in 0..6 {
        package[i] = 0xff;
    };

    for i in 0..6 {
        let y = u8::from_str_radix(mac.get(i*2..(i*2 + 2)).unwrap(), 16);
        if let Ok(w) = y {
            package[i + 6] = w;
        } else {
            err = true;
            break
        }
    }

    for e in 0..15 {
        let pos = (e + 2) * 6;
        for i in 0..6 {
            let y = u8::from_str_radix(mac.get(i*2..(i*2 + 2)).unwrap(), 16);
            if let Ok(w) = y {
                package[(i) + pos] = w;
            } else {
                err = true;
                break
            }
        }
    }

    if err {
        Err(false)
    } else {
        Ok(package)
    }
    
}

pub fn send_package(addr: std::net::SocketAddr, mac: String) -> Result<bool,usize> {
    let com =  UdpSocket::bind("0.0.0.0:0");
    if let Ok(socket) = com {
        socket.set_broadcast(true).unwrap();
        if let Ok(magic_package) = generate_magic_package(&mac) {
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

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct CollectedIpTargets {
    pub socket_addr: std::net::SocketAddr,
    pub mac: String
}

fn mac_address_correct(mac: String) -> Result<String, bool> {
    if mac.len() != 17 {
        Err(false)
    } else if mac.split("-").collect::<String>().len() != 12 {
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