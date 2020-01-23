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

/**
 * The exported FFI version of [[GravTree]]'s `new()` method. Returns a void pointer to the location
 * in memory where the [[GravTree]] is located. Use this void pointer to tell Rust where to look for
 * the tree in the other FFI functions.
 */
void *new(const Entity *array, int length, double time_step);

/**
 * The exported FFI version of [[GravTree]]'s `time_step()` method. Instead of being a method, it is a
 * function which takes in a [[GravTree]] (rather, a void pointer to the space where the [[GravTree]] is).
 */
void *time_step(void *grav_tree_buf);
