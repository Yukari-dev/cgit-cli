#include "device.h"
#include <stdio.h>

pcap_t *open_device(const char* device, const char* filter){
    char error_buf[PCAP_ERRBUF_SIZE];
    pcap_t *handle = pcap_open_live(device, 65535, 1, 1000, error_buf);
    if(handle == NULL){
        return NULL;
    }
    struct bpf_program fp;
    int result = pcap_compile(handle, &fp, filter, 1, PCAP_NETMASK_UNKNOWN);
    if(result == PCAP_ERROR){
        fprintf(stderr, "Compile error: %s\n", pcap_geterr(handle));
        pcap_close(handle);
        return NULL;
    }
    result = pcap_setfilter(handle, &fp);
    if(result == PCAP_ERROR){
        fprintf(stderr, "Filter error: %s\n", pcap_geterr(handle));
        pcap_close(handle);
        return NULL;
    }
    pcap_freecode(&fp); 
    return handle;
}

void close_device(pcap_t *handle){
    pcap_close(handle);
}
