#include "stats.h"
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

StatsTable *create_stat(){
    StatsTable *table = malloc(sizeof(StatsTable));
    if(table == NULL) {
        fprintf(stderr, "Error: could not allocate stats table\n");
        return NULL;
    }
    memset(table, 0, sizeof(StatsTable));
    return table;
}

void update_stats(StatsTable *table, const char* ip, int bytes, int protocol){
    if(protocol == 6)  table->tcp_count++;
    if(protocol == 17) table->udp_count++;
    if(protocol == 1)  table->icmp_count++;
    for(int i = 0; i < table->count; i++){
        char* currentIp = table->entries[i].ip;
        if(strcmp(currentIp, ip) == 0){
            table->entries[i].packet_count++;
            table->entries[i].bytes_total += bytes;
            return;
        } 
    }
    strcpy(table->entries[table->count].ip, ip);
    table->entries[table->count].bytes_total = bytes;
    table->entries[table->count].packet_count = 1;
    table->count++;
}

int stats_total_packets(StatsTable *stats){
    int total = 0;
    for(int i = 0; i < stats->count; i++){
        total += stats->entries[i].packet_count;
    }
    return total;
}

long stats_total_bytes(StatsTable *stats){
    long total = 0;
    for(int i = 0; i < stats->count; i++){
        total += stats->entries[i].bytes_total;
    }
    return total;
}

void print_stats(StatsTable *table){
    /*for(int i = 0; i < table->count; i++){
        printf("%-20s  packets: %d  bytes: %ld\n", 
            table->entries[i].ip,
            table->entries[i].packet_count,
            table->entries[i].bytes_total
        );
    }
    printf("TCP protocol count: %d\n, 
            UDP protocol count: %d\n, 
            ICMP protocol count: %d\n",
        table->tcp_count,
        table->udp_count,
        table->icmp_count
    ); */
}

void free_stats(StatsTable *table){
    if(table != NULL){
        free(table);
    }
}