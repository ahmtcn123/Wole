# Wol
Wake-On-Lan CLI

```ps
wol.exe --help
```

## CLI Usage

- ### Waking up single device

```ps
    wol --mac 2c-2c-2c-2c-2c-2c --ip 192.168.1.100
```

- ### Waking up multiple devices

```ps
    wol --mac 2c-2c-2c-2c-2c-2c --ip 192.168.1.100  --mac 2a-2c-2d-2b-2c-2c --ip 192.168.1.102
```

- ### Sent packages aggressively

```ps
    wol --mac 2c-2c-2c-2c-2c-2c --ip 192.168.1.100 --aggressive
```

## API Usage

```rust
    let create_package = lib::generate_magic_package("2C2C2C2C2C2C");

    if let Ok(package) = create_package {
        println!("Package Created");

        let send_package = lib::send_package(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080), package);

        if let Ok(_) = send_package {
            println!("Package sent")
        } else if let Err(code) = send_package {
            if code == 0 {
                println!("Failed to connect device");
            } else if code == 1 {
                println!("Failed to send packages");
            }
        }
    } else if let Err(_) = create_package {
        println!("Failed to create package")
    }
```