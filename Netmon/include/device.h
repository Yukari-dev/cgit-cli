#ifndef DEVICE_H
#define DEVICE_H
#include <pcap.h>

pcap_t *open_device(const char* device, const char* filter);
void close_device(pcap_t *handle);

#endif