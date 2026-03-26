def on_packet(src_ip, dst_ip, protocol, size):
    with open("netmon.log", "a") as f:
        f.write(f"{src_ip} -> {dst_ip} [{protocol}] {size} bytes\n")