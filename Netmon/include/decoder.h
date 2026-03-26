#ifndef DECODER_H
#define DECODER_H
#define PROTO_TCP 6
#define PROTO_UDP 17
#define PROTO_ICMP 1
#include <pcap.h>

void decoder(unsigned char *user, const struct pcap_pkthdr *header, const unsigned char *packet);

#endif