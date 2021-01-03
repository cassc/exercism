#include "difference_of_squares.h"
#include <math.h>

unsigned int sum_of_squares(unsigned int number){
  unsigned int r = 0;
  for (unsigned int i = 1; i <=number; i++){
    r += pow(i, 2);
  }
  return r;
}
unsigned int square_of_sum(unsigned int number) {
  unsigned int r = 0;
  for (unsigned int i = 1; i <= number; i++){
    r += i;
  }
  return pow(r, 2);
}

unsigned int difference_of_squares(unsigned int number){
  return square_of_sum(number) - sum_of_squares(number);
}
