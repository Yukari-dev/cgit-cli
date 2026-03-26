#ifndef STATS_H
#define STATS_H

typedef struct{
    char ip[16];
    int packet_count;
    long bytes_total;
} IPStats;

typedef struct{
    IPStats entries[256];
    int count;
    int tcp_count;
    int udp_count;
    int icmp_count;
} StatsTable;

StatsTable *create_stat();

void update_stats(StatsTable *table, const char *ip, int bytes, int protocol);

int stats_total_packets(StatsTable *stats);

long stats_total_bytes(StatsTable *stats);

void print_stats(StatsTable *table);

void free_stats(StatsTable *table);

#endif