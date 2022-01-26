#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <inttypes.h>
#include <sys/types.h>
#include <sys/queue.h>
#include <netinet/in.h>
#include <setjmp.h>
#include <stdarg.h>
#include <ctype.h>
#include <errno.h>
#include <getopt.h>
#include <signal.h>
#include <stdbool.h>

#include <rte_common.h>
#include <rte_log.h>
#include <rte_malloc.h>
#include <rte_memory.h>
#include <rte_memcpy.h>
#include <rte_eal.h>
#include <rte_launch.h>
#include <rte_atomic.h>
#include <rte_cycles.h>
#include <rte_prefetch.h>
#include <rte_lcore.h>
#include <rte_per_lcore.h>
#include <rte_branch_prediction.h>
#include <rte_interrupts.h>
#include <rte_random.h>
#include <rte_debug.h>
#include <rte_ether.h>
#include <rte_ethdev.h>
#include <rte_mempool.h>
#include <rte_mbuf.h>
#include <rte_string_fns.h>

//eth_headerのため
#include <netinet/if_ether.h>


//filterのため
#include "filter.h"
#include <unistd.h>
#include <rte_byteorder.h>

int main(){
    void *map = NULL;

    printf("generating map now.....");
	map = gen_map();

    Mapbool src_mapbool;
    uint64_t val;
    val = ((uint64_t)192 << 16) + ((uint64_t)168 << 24) + ((uint64_t)1 << 32) + ((uint64_t)1 << 40) + 454;
    
    while (1)
    {
        printf("\n\n5sec start\n");
        sleep(5);
        printf("5sec end\n");
        src_mapbool = read_map(map, val, 1);
        printf("%d", rte_cpu_to_be_16(rte_cpu_to_be_16(src_mapbool.port)));

    }
    
}