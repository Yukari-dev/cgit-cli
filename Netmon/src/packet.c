#include "packet.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>


Packet *create_packet(const unsigned char *data, int length){
    Packet *packet = malloc(sizeof(Packet));
    if(packet == NULL){
        fprintf(stderr, "Error: packet pointer is null\n");
        return NULL;
    }
    packet->data = malloc(sizeof(unsigned char) * length);
    if(packet->data == NULL){
        fprintf(stderr, "Error: data pointer is null\n");
        free_packet(packet);
        return NULL;
    }
    memcpy(packet->data, data, length);
    packet->length = length;
    return packet;
}

void free_packet(Packet* p){
    if(p != NULL){
        free(p->data);
        free(p);
    }
}