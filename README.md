# Probe
Probe is a tiny cURL-like tool with support for HTTPS, Websocket, an TCP.



## Building
Probe can be built with the `cargo` tool.
```shell
# Debug mode
cargo build

# Release mode
cargo build --release
```

## Usage
### HTTP
```shell
probe http -X GET --header "Content-Type: application/json" https://httpbin.org/status/200 
```

### Websocket
This starts an interactive session.
```shell
probe web-socket wss://echo.websocket.org
```

### TCP
This starts an interactive session.
```shell
probe tcp tcpbin.com:4242
```