#ifndef UI_H
#define UI_H
#include "stats.h"
#include "packet.h"
#include <ncurses.h>

extern WINDOW *header_win;
extern WINDOW *feed_win;
extern WINDOW *stats_win;
extern WINDOW *footer_win;

void ui_init(const char* device, const char* filter);

void ui_update_feed(const char* src, const char* dest, const char* protocol, int bytes, uint16_t src_port, uint16_t dst_port);

void ui_update_stats(CaptureContext *ctx);

void ui_update_footer(CaptureContext *ctx);

int compare_entries(const void *a, const void *b);

void ui_cleanup(void);

#endif