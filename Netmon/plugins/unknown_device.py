KNOWN_DEVICES = {
    "192.168.1.199",
    "192.168.1.1",
}

seen = set()

def on_packet(src_ip, dst_ip, protocol, size):
    if src_ip.startswith("192.168.") and src_ip not in KNOWN_DEVICES:
        if src_ip not in seen:
            seen.add(src_ip)
            with open("netmon.log", "a") as f:
                f.write(f"{src_ip} -> {dst_ip} [{protocol}] {size} bytes\n")