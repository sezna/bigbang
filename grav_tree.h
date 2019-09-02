#include "stdint.h"

typedef struct Node Node;

typedef struct GravTree {
  Node root;
  uintptr_t number_of_entities;
  /**
   * The tolerance for the distance from an entity to the center of mass of an entity
   * If the distance is beyond this threshold, we treat the entire node as one giant
   * entity instead of recursing into it.
   */
  double theta;
  int32_t max_pts;
  /**
   * The length of time that passes each step. This coefficient is multiplied by the velocity
   * before the velocity is added to the position of the entities each step.
   */
  double time_step;
} GravTree;

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
  double theta;
  double time_step;
} Entity;

GravTree from_data_file(const char *file_path_buff, double theta, int max_pts, double time_step);

GravTree new(const Entity *array, int length, double theta, int max_pts, double time_step);

GravTree time_step(GravTree gravtree);

void write_data_file(const char *file_path_buff, GravTree gravtree);
