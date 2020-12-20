#include <stdlib.h>
#include <stdio.h>
#include "resistor_color.h"
#include <stdbool.h>

static resistor_band_t bands[LAST];

bool initiazed = false;

resistor_band_t *colors() {
  if (initiazed){
    return bands;
  }

  for (resistor_band_t color = BLACK; color < LAST; color++){
    bands[color] = color;
  }

  initiazed = true;
  return bands;
}

int color_code(resistor_band_t code){
  return (int) code;
}
