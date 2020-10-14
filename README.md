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

Soon