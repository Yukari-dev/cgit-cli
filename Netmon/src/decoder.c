#include "decoder.h"
#include "packet.h"
#include "ui.h"
#include "plugins.h"
#include <netinet/ip.h>
#include <stdio.h>
#include <string.h>
#include <arpa/inet.h>
#include <netinet/tcp.h>
#include <netinet/udp.h>

void decoder(unsigned char *user, const struct pcap_pkthdr *header, const unsigned char *packet){
    unsigned short type = (packet[12] << 8) | packet[13];

    if(type != 0x0800) return;

    CaptureContext *ctx = (CaptureContext*)user;
    Packet *pkt = create_packet(packet, header->caplen);
    if(pkt == NULL) return;
    unsigned char *packet_data = pkt->data;

    struct iphdr *ip = (struct iphdr*)(packet_data + 14);
    struct in_addr srcIp;
    srcIp.s_addr = ip->saddr;

    struct in_addr destIp;
    destIp.s_addr = ip->daddr;
    
    unsigned char *transport = packet_data + 14 + (ip->ihl * 4);

    uint16_t src_port = 0;
    uint16_t dst_port = 0;


    char* protocol = "";
    if(ip->protocol == PROTO_TCP) {
        struct tcphdr *tcp = (struct tcphdr*)transport;
        src_port  = ntohs(tcp->source);
        dst_port  = ntohs(tcp->dest);
        protocol = "TCP";
    }
    if(ip->protocol == PROTO_UDP) {
        struct udphdr *udp = (struct udphdr*)transport;
        src_port  = ntohs(udp->source);
        dst_port  = ntohs(udp->dest);
        protocol = "UDP";
    }
    if(ip->protocol == PROTO_ICMP) protocol = "ICMP";
    char src_ip[16];
    strncpy(src_ip, inet_ntoa(srcIp), 15);
    int bytes = header->caplen;
    /*printf("source Ip %s -> destination Ip %s Protocol %s, Length %d bytes\n", 
        src_ip, 
        inet_ntoa(destIp), 
        protocol, 
        bytes
    ); */

    
    char dest_ip[16];
    strncpy(dest_ip, inet_ntoa(destIp), 15);

    pthread_mutex_lock(&ctx->stats_mutex);
    update_stats(ctx->stats, src_ip, bytes, ip->protocol);
    update_stats(ctx->stats, dest_ip, bytes, ip->protocol);
    pthread_mutex_unlock(&ctx->stats_mutex);

    pthread_mutex_lock(&ctx->ncurses_mutex);
    ui_update_feed(src_ip, dest_ip, protocol, bytes, src_port, dst_port);
    pthread_mutex_unlock(&ctx->ncurses_mutex);

    plugins_on_packet(src_ip, dest_ip, protocol, bytes);
    free_packet(pkt);
}
