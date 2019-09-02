#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>


struct GravTree;

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
  double theta;
  double time_step;
};

extern "C" {

GravTree from_data_file(const char *file_path_buff, double theta, int max_pts, double time_step);

GravTree new(const Entity *array, int length, double theta, int max_pts, double time_step);

GravTree time_step(GravTree gravtree);

void write_data_file(const char *file_path_buff, GravTree gravtree);

} // extern "C"

