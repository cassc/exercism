#include "isogram.h"
#include <ctype.h>

bool is_isogram(const char phrase[]){
  if (!phrase){
    return false;
  }

  int i = 0, code;
  int r[256] = {0};

  while ((code = phrase[i++])) {
    code = tolower(code);
    if (code!= '-' && code != ' ' && r[code]){
      return false;
    }
    r[code] = 1;
  }
  return true;
}
