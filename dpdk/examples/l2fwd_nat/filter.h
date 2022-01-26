#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Mapbool {
  uint32_t ip;
  uint16_t port;
  const void *y;
  bool z;
} Mapbool;

const void *gen_map(void);

struct Mapbool read_map(const void *map, uint64_t src_ip_port, int32_t t);
