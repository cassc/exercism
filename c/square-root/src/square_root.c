#include "square_root.h"


unsigned int square_root(unsigned int s){
  int result = s / 4;
  while (!found(result, s)){
    result = next_guess(result, s);
  }
  return result;
}

bool found(unsigned int result, unsigned int s) {
  return !(result * result - s);
}

unsigned int next_guess(unsigned int result, unsigned int s){
  if (result){
      return (result + s / result) / 2;
  } else{
    return 1;
  }
}
