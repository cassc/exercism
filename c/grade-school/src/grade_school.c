#include "grade_school.h"
#include <stdint.h>
#include <string.h>

static roster_t roster = { 0 };

bool compare_student(student_t *a, student_t *b){
  if (a->grade != b->grade){
    return a->grade > b->grade;
  }
  return strcmp(a->name, b->name) >= 0;
}

bool add_student(char *name, uint8_t grade){
  if (roster.count >= MAX_STUDENTS){
    return false;
  }

  student_t *students = roster.students;
  student_t st = (student_t){grade, name};

  int i;
  for (i = roster.count - 1; i >= 0 && compare_student(students+i, &st); i--) {
    *(students + i + 1) = *(students + i);
  }
  *(students + i + 1) = st;

  roster.count++;

  return true;
}

void clear_roster(){
  roster.count = 0;
}

roster_t get_grade(uint8_t grade){
  roster_t result = {0};
  uint8_t j = 0;
  for (uint8_t i = 0; i<roster.count; i++){
    if (grade == roster.students[i].grade){
      result.count++;
      result.students[j++] = roster.students[i];
    }
  }

  return result;
}

roster_t get_roster(){
  return roster;
}
