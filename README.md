echo server

## usage

```shell
Usage: echo-server.exe [OPTIONS]

Options:
  -p, --port <PORT>                          [default: 34567]
      --ipv6
  -t, --threads <THREADS>                    [default: 1]
  -m, --min-response-len <MIN_RESPONSE_LEN>  [default: 1]
  -v, --verbose
  -h, --help                                 Print help
  -V, --version                              Print version
```

## local build

requirement: install [zig](https://ziglang.org/), add to PATH

```shell
cargo install cargo-zigbuild
# windows x64
cargo build --target x86_64-pc-windows-msvc --release
# linux x64(use zig)
cargo zigbuild --target x86_64-unknown-linux-musl --release
```

## test(Linux)

ip-V4

```shell
# udp test port
nc -4vuz 127.0.0.1 34567
# udp send  msg
echo "ping" | nc -4vu 127.0.0.1 34567
# tcp test port
nc -4vz 127.0.0.1 34567
# tcp send  msg
echo "ping" | nc -4v 127.0.0.1 34567
```

ip-V6

```shell
# udp test port
nc -6vuz ::1 34567
# udp send  msg
echo "ping" | nc -4vu ::1 34567
# tcp test port
nc -6vz 127.0.0.1 34567
# tcp send  msg
echo "ping" | nc -6v ::1 34567
```

## test(Powershell)

IPV4

```powershell
# UDP test port
Test-NetConnection -ComputerName "127.0.0.1" -Port 34567 -Protocol Udp -InformationLevel Detailed

# UDP send msg
$client = New-Object System.Net.Sockets.UdpClient
$data = [System.Text.Encoding]::ASCII.GetBytes("ping")
$client.Send($data, "127.0.0.1", 34567)
$client.Close()

# TCP test port
Test-NetConnection -ComputerName "127.0.0.1" -Port 34567 -Protocol Tcp

# TCP send msg
$client = New-Object System.Net.Sockets.TcpClient
$client.Connect("127.0.0.1", 34567)
$stream = $client.GetStream()
$data = [System.Text.Encoding]::ASCII.GetBytes("ping")
$stream.Write($data, 0, $data.Length)
$client.Close()
```

IPV6

```powershell
# UDP test port
Test-NetConnection -ComputerName "::1" -Port 34567 -Protocol Udp -InformationLevel Detailed

# UDP send msg
$client = New-Object System.Net.Sockets.UdpClient -ArgumentList ([System.Net.Sockets.AddressFamily]::InterNetworkV6)
$data = [System.Text.Encoding]::ASCII.GetBytes("ping")
$client.Send($data, "::1", 34567)
$client.Close()

# TCP test port
Test-NetConnection -ComputerName "::1" -Port 34567 -Protocol Tcp

# TCP send msg
$client = New-Object System.Net.Sockets.TcpClient -ArgumentList ([System.Net.Sockets.AddressFamily]::InterNetworkV6)
$client.Connect("::1", 34567)
$stream = $client.GetStream()
$data = [System.Text.Encoding]::ASCII.GetBytes("ping")
$stream.Write($data, 0, $data.Length)
$client.Close()
```