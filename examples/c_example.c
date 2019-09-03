#include "../grav_tree.h"

// Run me with "make run" from the project root!

int main(void) {
  void * grav_tree = from_data_file("./test_files/test_input.txt", 0.2, 3, 0.2);
  void * next_step = time_step(grav_tree);
  return 0;
}
