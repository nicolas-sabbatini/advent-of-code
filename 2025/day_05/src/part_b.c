#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_LINE_LENGTH 1024

#define Vec(T)                                                                 \
  typedef struct {                                                             \
    T *items;                                                                  \
    size_t size;                                                               \
    size_t capacity;                                                           \
  } Vec_##T
#define VecPrint(vec, format)                                                  \
  do {                                                                         \
    printf("[ ");                                                              \
    for (size_t i = 0; i < vec.size; i++) {                                    \
      printf(format, vec.items[i]);                                            \
      printf(" ");                                                             \
    }                                                                          \
    printf("]\n");                                                             \
  } while (0)
#define VecPush(vec, element)                                                  \
  do {                                                                         \
    if (vec.size >= vec.capacity) {                                            \
      if (vec.capacity == 0)                                                   \
        vec.capacity = 10;                                                     \
      else                                                                     \
        vec.capacity *= 2;                                                     \
    }                                                                          \
    vec.items = realloc(vec.items, vec.capacity * sizeof(*vec.items));         \
    vec.items[vec.size++] = element;                                           \
  } while (0)
#define VecFree(vec)                                                           \
  do {                                                                         \
    vec.size = 0;                                                              \
    vec.capacity = 0;                                                          \
    free(vec.items);                                                           \
    vec.items = NULL;                                                          \
  } while (0)
#define VecFlush(vec)                                                          \
  do {                                                                         \
    vec.size = 0;                                                              \
  } while (0)

char *parse(char *start, const char *split, char **needle) {
  char *s = *needle == NULL ? start : *needle;
  if (*s != '\0') {
    size_t slice_size = strcspn(s, split);
    *needle = s + slice_size;
    size_t limit = strspn(*needle, split);
    for (size_t i = 0; i < limit; i++) {
      *needle[i] = '\0';
    }
    *needle += limit;
    return s;
  }
  return NULL;
}

typedef struct {
  size_t min;
  size_t max;
} Range;
Vec(Range);

int compare_mins(const void *a, const void *b) {
  return (((Range *)a)->min >= ((Range *)b)->min);
}

int main(int argc, char *argv[argc + 1]) {
  char *file_name = argv[1];
  char buffer[MAX_LINE_LENGTH];
  FILE *fp = fopen(file_name, "r");
  Vec_Range input = {};
  size_t out = 0;
  while (fgets(buffer, MAX_LINE_LENGTH, fp) != NULL) {
    if (buffer[0] == '\n') {
      break;
    }
    char *dash_needle = NULL;
    char *left = parse(buffer, "-", &dash_needle);
    char *right = parse(buffer, "-", &dash_needle);
    Range new_range = {
        .min = strtol(left, NULL, 10),
        .max = strtol(right, NULL, 10),
    };
    VecPush(input, new_range);
  }
  bool scuash;
  Vec_Range scuasher = {};
  do {
    scuash = false;
    qsort(input.items, input.size, sizeof(Range), compare_mins);
    VecPush(scuasher, input.items[0]);
    for (size_t i = 1; i < input.size; i++) {
      size_t ti = scuasher.size - 1;
      if (scuasher.items[ti].max < input.items[i].min) {
        VecPush(scuasher, input.items[i]);
      } else if (scuasher.items[ti].max >= input.items[i].min &&
                 scuasher.items[ti].max >= input.items[i].max) {
        scuash = true;
      } else if (scuasher.items[ti].max >= input.items[i].min &&
                 scuasher.items[ti].max < input.items[i].max) {
        input.items[i].min = scuasher.items[ti].max + 1;
        VecPush(scuasher, input.items[i]);
      } else {
        printf("Unexpected!\n");
        exit(EXIT_FAILURE);
      }
    }
    Vec_Range temp = scuasher;
    scuasher = input;
    input = temp;
    VecFlush(scuasher);
  } while (scuash);
  VecFree(scuasher);
  for (size_t i = 0; i < input.size; i++) {
    out += (input.items[i].max - input.items[i].min) + 1;
  }
  fclose(fp);
  VecFree(input);
  printf("Res: %zu\n", out);
  return EXIT_SUCCESS;
}
