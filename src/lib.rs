use std::str;

pub fn generate_magic_package(mac: &str) -> Result<[u8; 102], bool> {
    let mut err = false;
    let mut package :[u8; 102] = [0; 102];

    'outer: for e in 0..17 {
        for i in 0..6 {
            if e == 0 {
                package[i] = 0xff;
            } else {
                if let Ok(w) = u8::from_str_radix(mac.get(i*2..i*2+2).unwrap(), 16) {
                    package[i + e * 6] = w;
                } else {
                    err = true;
                    break 'outer;
                }   
            }
        }
    }

    if err {
        Err(false)
    } else {
        Ok(package)
    }
}