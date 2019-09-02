#include "../grav_tree.h"

int main(void) {
  GravTree grav_tree = from_data_file("../test_files/test_input.txt", 0.2, 3, 0.2);
  return 0;
}
