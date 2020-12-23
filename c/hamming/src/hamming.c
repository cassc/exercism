#include "hamming.h"
#include <stdio.h>


int compute(const char *lhs, const char *rhs){
  if (!lhs || !rhs){
    return -1;
  }

  int dist = 0;
  for (int i=0; ; i++){
    char l = *lhs++;
    char r = *rhs++;

    if (!l && !r){
      return dist;
    }

    if (!l || !r){
      return -1;
    }

    if (l != r){
      dist++;
    }
  }
  return dist;
}
