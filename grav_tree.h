#include "stdint.h"

/**
 * An Entity is an object (generalized to be spherical, having only a radius dimension) which has
 * velocity, position, radius, and mass. This gravitational tree contains many entities and it moves
 * them around according to the gravity they exert on each other.
 */
typedef struct Entity {
  double vx;
  double vy;
  double vz;
  double x;
  double y;
  double z;
  double radius;
  double mass;
} Entity;

void *from_data_file(const char *file_path_buff);

void *new(const Entity *array, int length);

void *time_step(void *gravtree_buf);

void write_data_file(const char *file_path_buff, unsigned char *gravtree_buf);
