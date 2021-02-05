#include "binary_search.h"
#include <stdio.h>

int *binary_search(int value, int *arr, size_t length){
  int left = 0;
  int right = length - 1;
  int mid = (right + left) / 2;

  while (left <= right) {
    mid = (right + left) / 2;
    int v = *(arr + mid);
    if (value < v){
      right = mid - 1;
    } else if (value > v){
      left = mid + 1;
    } else {
      return arr + mid;
    }
  }

  return 0;
}
