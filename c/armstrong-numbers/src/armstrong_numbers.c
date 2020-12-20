#include "armstrong_numbers.h"
#include <math.h>
#include <stdlib.h>
#include <stdio.h>

bool is_armstrong_number(int candidate){
  int num_digits = 0;
  int n = candidate;

  while(n > 0){
    n /= 10;
    num_digits++;
  }

  int result = 0;
  n = candidate;
  int rem = 0;

  while(n > 0){
    rem = n % 10;
    n /= 10;
    result += pow(rem, num_digits);
  }

  return result == candidate;
}
