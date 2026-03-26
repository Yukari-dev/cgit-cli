<div align="center">

# 🔍 netmon

**A real-time network traffic monitor for Linux**

*Live ncurses dashboard · Python plugin system · Zero dependencies beyond libpcap*

<img width="1344" height="670" alt="netmon dashboard" src="https://github.com/user-attachments/assets/b79755e8-846a-4b70-ba86-6d5533813866" />

</div>

---

## ✨ Features

| | Feature |
|---|---|
| 📡 | Live packet capture using libpcap |
| 🔬 | Protocol decoding — Ethernet, IP, TCP, UDP, ICMP |
| 📊 | Real-time ncurses dashboard with live feed and top talkers |
| 📈 | Protocol breakdown statistics |
| ⏱️ | Elapsed time, total bytes and packet counter |
| 🐍 | Python plugin system — drop a `.py` file, it runs on every packet |
| ⌨️ | Clean exit with `q`, `Esc`, or `Ctrl+C` |

---

## 🚀 Getting Started

### Requirements
- Linux
- `libpcap`
- `ncurses`
- `Python 3.14`

### Install dependencies
```bash
# Arch / CachyOS
sudo pacman -S libpcap ncurses python
```

### Build
```bash
make
```

### Run
```bash
sudo ./bin/netmon
# or
make run
```

---

## ⚙️ Usage
```bash
sudo ./bin/netmon [-i interface] [-f filter] [-c count]
```

| Flag | Description | Default |
|------|-------------|---------|
| `-i` | Network interface | `wlan0` |
| `-f` | BPF filter expression | none |
| `-c` | Packet limit (`-1` = infinite) | `-1` |

### Examples
```bash
# capture everything
sudo ./bin/netmon

# capture only TCP traffic
sudo ./bin/netmon -f "tcp"

# use a specific interface
sudo ./bin/netmon -i eth0

# capture only 100 packets
sudo ./bin/netmon -c 100
```

---

## 📁 Project Structure
```
netmon/
├── src/
│   ├── netmon.c     — entry point, CLI, threads
│   ├── packet.c     — packet memory management
│   ├── device.c     — libpcap interface
│   ├── decoder.c    — protocol decoding
│   ├── stats.c      — statistics engine
│   ├── ui.c         — ncurses dashboard
│   └── plugins.c    — Python plugin engine
├── include/         — header files
├── plugins/         — Python plugins
├── obj/             — object files
├── bin/             — compiled binary
└── Makefile
```

---

## 🐍 Plugin System

Drop a `.py` file into the `plugins/` directory. It must define an `on_packet` function:
```python
def on_packet(src_ip, dst_ip, protocol, size):
    # src_ip   — source IP address string
    # dst_ip   — destination IP address string
    # protocol — "TCP", "UDP", or "ICMP"
    # size     — packet size in bytes
    pass
```

> Plugins are loaded automatically at startup. No recompilation needed.

---

## 📦 Example Plugins

### 📝 logger.py
Logs all traffic to `netmon.log`

<img width="1442" height="406" alt="loggerCode" src="https://github.com/user-attachments/assets/99649712-880f-4a60-b292-b590cf1d9d56" />

---

### 📶 bandwidth_alert.py
Alerts when a host transfers more than 1MB in 10 seconds

<img width="2028" height="938" alt="bandwidthCode" src="https://github.com/user-attachments/assets/1bd75fe0-a62c-4dc0-a372-522527740e2b" />

---

### 🔎 port_scan.py
Detects potential port scanning activity

<img width="1424" height="1014" alt="portScanCode" src="https://github.com/user-attachments/assets/41c43b44-e8f0-480e-92d6-86fc54a043ea" />

---

### 👻 unknown_device.py
Alerts when an unknown device appears on your network

<img width="1592" height="786" alt="UnknownCode" src="https://github.com/user-attachments/assets/a878dcc9-63e9-4198-97da-a2aadc892f40" />

---

## 📄 License

MIT — do whatever you want with it.