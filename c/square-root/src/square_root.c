#include "square_root.h"


unsigned int square_root(unsigned int s){
  int result = s / 4 || 1;

  while (result * result - s){
    result = (result + s / result) / 2;
  }

  return result;
}
