#include "device.h"
#include "packet.h"
#include "decoder.h"
#include "stats.h"
#include "ui.h"
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <pthread.h>
#include <unistd.h>
#include <time.h>
#include <signal.h>
#include "plugins.h"

volatile int running = 1;
pcap_t *g_handle;

void handle_signal(int sig) { 
    running = 0;
    if(g_handle) pcap_breakloop(g_handle);
}

void *ui_thread(void *arg){
    CaptureContext *ctx = (CaptureContext*)arg;
    while(running){
        int ch = getch();
        if(ch == 'q' || ch == 27){
            running = 0;
            pcap_breakloop(ctx->handle);
            break;
        }
        pthread_mutex_lock(&ctx->ncurses_mutex);
        ui_update_stats(ctx);
        ui_update_footer(ctx);
        pthread_mutex_unlock(&ctx->ncurses_mutex);
        sleep(1);
    }
    return NULL;
}


int main(int argc, char* argv[]){
    time_t start_time = time(NULL);
    const char *device = "wlan0";
    const char *filter = "";
    int packets_limit  = -1;


    for(int i = 1; i < argc; i++){
        if(strcmp(argv[i], "-i") == 0){
            if(i + 1 >= argc){
                fprintf(stderr, "Error: -i requires an argument\n");
                return 1;
            }
            i++;
            device = argv[i];
        } else if(strcmp(argv[i], "-f") == 0){
            if(i + 1 >= argc){
                fprintf(stderr, "Error: -f requires an argument\n");
                return 1;
            }
            i++;
            filter = argv[i];
        } else if(strcmp(argv[i], "-c") == 0){
            if(i + 1 >= argc){
                fprintf(stderr, "Error: -c requires an argument\n");
                return 1;
            }
            i++;
            packets_limit = atoi(argv[i]);
        } else if(strcmp(argv[i], "-h") == 0){
            printf("Usage: netmon [-i interface] [-f filter] [-c count]\n");
            return 0;
        }
    }

    plugins_init("plugins");
    ui_init(device, filter);
    g_handle = open_device(device, filter);
    if(g_handle == NULL){
        fprintf(stderr, "Couldn't open device %s\n", device);
        return 1;
    }
    
    
    //PacketBuffer buffer;
    //buffer.packets = malloc(sizeof(Packet*) * packets_limit);
    //buffer.count = 0;
    
    CaptureContext ctx;
    //ctx.buffer = &buffer;
    ctx.stats  = create_stat();
    ctx.handle = g_handle;
    ctx.start_time = start_time;
    pthread_mutex_init(&ctx.stats_mutex, NULL);
    pthread_mutex_init(&ctx.ncurses_mutex, NULL);
    signal(SIGINT, handle_signal);
    pthread_t thread;
    pthread_create(&thread, NULL, ui_thread, &ctx);   
    pcap_loop(g_handle, packets_limit, decoder, (unsigned char*)&ctx);
    running = 0;
    pthread_join(thread, NULL);
    ui_cleanup();
    print_stats(ctx.stats);
    close_device(g_handle);

    //for(int i = 0; i < ctx.buffer->count; i++){
    //    free_packet(ctx.buffer->packets[i]);
    //}
    //free(ctx.buffer->packets);
    pthread_mutex_destroy(&ctx.stats_mutex);
    pthread_mutex_destroy(&ctx.ncurses_mutex);
    free_stats(ctx.stats);
    plugins_cleanup();

    return 0;
}
