import time

tracker = {}

THRESHOLD = 1_000_000

def on_packet(src_ip, dst_ip, protocol, size):
    now = time.time()
    if src_ip not in tracker:
        tracker[src_ip] = {"bytes": 0, "since": now}
    tracker[src_ip]["bytes"] += size
    if now - tracker[src_ip]["since"] > 10:
        tracker[src_ip] = {"bytes": 0, "since": now}
    if tracker[src_ip]["bytes"] > THRESHOLD:
        with open("netmon.log", "a") as f:
            f.write(f"[ALERT] Bandwidth hog: {src_ip} used {tracker[src_ip]['bytes']} bytes in 10s\n")
        tracker[src_ip] = {"bytes": 0, "since": now}