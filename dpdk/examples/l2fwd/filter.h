#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Mapbool {
  bool x;
  const void *y;
} Mapbool;

bool ip_add(uint8_t *x);

const void *gen_map(void);

struct Mapbool read_map(const void *map, uint8_t *mac);

const void *check_map(const void *map);
