#include "difference_of_squares.h"
#include <math.h>

unsigned int sum_of_squares(unsigned int n){
  unsigned int r = n * (n + 1) * (2 * n + 1) / 6;
  return r;
}
unsigned int square_of_sum(unsigned int n) {
  unsigned int r = n * (n + 1) / 2;
  return r * r;
}

unsigned int difference_of_squares(unsigned int number){
  return square_of_sum(number) - sum_of_squares(number);
}
