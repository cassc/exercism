#ifndef GRADE_SCHOOL_H
#define GRADE_SCHOOL_H

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>

#define MAX_NAME_LENGTH 20
#define MAX_STUDENTS 20

typedef struct {
   uint8_t grade;
   char *name;
} student_t;

typedef struct {
   size_t count;
   student_t students[MAX_STUDENTS];
} roster_t;

roster_t get_grade(uint8_t grade);
bool add_student(char *name, uint8_t grade);
void clear_roster();
roster_t get_roster();
bool compare_student(student_t *a, student_t *b);

#endif
