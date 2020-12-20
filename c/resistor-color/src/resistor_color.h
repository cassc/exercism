#ifndef RESISTOR_COLOR_H
#define RESISTOR_COLOR_H

typedef enum {
  BLACK, BROWN, RED, ORANGE, YELLOW,
  GREEN, BLUE, VIOLET, GREY, WHITE, LAST
} resistor_band_t;

/**
 * Convert `resistor_band_t` to an array of colors
 */
resistor_band_t* colors();

/**
 * Get integer value from color code
 */
int color_code(resistor_band_t code);
#endif
