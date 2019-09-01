#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>


struct Node;

struct String;

struct GravTree {
  Node root;
  uintptr_t number_of_entities;
};

/// An Entity is an object (generalized to be spherical, having only a radius dimension) which has
/// velocity, position, radius, and mass. This gravitational tree contains many entities and it moves
/// them around according to the gravity they exert on each other.
struct Entity {
  double vx;
  double vy;
  double vz;
  double x;
  double y;
  double z;
  double radius;
  double mass;
};

extern "C" {

GravTree from_data_file(CString filepath);

GravTree new(const Entity *array, int length);

GravTree time_step(GravTree gravtree);

void write_data_file(String filepath, GravTree gravtree);

} // extern "C"

