import time

tracker = {}
THRESHOLD = 15;

def on_packet(src_ip, dst_ip, protocol, size):
    if protocol != "TCP": return

    now = time.time()

    if src_ip not in tracker:
        tracker[src_ip] = {"ports": set(), "since": now}
    if now - tracker[src_ip]["since"] > 5:
        tracker[src_ip] = {"ports": set(), "since": now}
    tracker[src_ip]["ports"].add(dst_ip)
    if len(tracker[src_ip]["ports"]) > THRESHOLD:
        with open("netmon.log", "a") as f:
            f.write(f"[ALERT] Port scan detected from {src_ip}\n")
        tracker[src_ip] = {"ports": set(), "since": now}