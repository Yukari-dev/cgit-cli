#ifndef PLUGINS_H
#define PLUGINS_H
#include <python3.14/Python.h>

void plugins_init(const char *plugins_dir);

void plugins_on_packet(const char *src_ip, const char *dest_ip, const char *protocol, int size);

void plugins_cleanup(void);

#endif