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
  size_t start;
  size_t end;
} Range;

Vec(Range);
Vec(size_t);

int main(int argc, char *argv[argc + 1]) {
  char *file_name = argv[1];
  char buffer[MAX_LINE_LENGTH];
  FILE *fp = fopen(file_name, "r");
  fgets(buffer, MAX_LINE_LENGTH, fp);
  fclose(fp);

  Vec_Range input = {};
  char *coma_needle = NULL;
  char *coma_split = parse(buffer, ",\n", &coma_needle);
  while (coma_split != NULL) {
    char *dash_needle = NULL;
    char *left = parse(coma_split, "-", &dash_needle);
    char *right = parse(coma_split, "-", &dash_needle);
    Range new_range = {.start = strtol(left, NULL, 10),
                       .end = strtol(right, NULL, 10)};
    VecPush(input, new_range);
    coma_split = parse(buffer, ",\n", &coma_needle);
  }

  Vec_size_t invalid_ids = {};
  char target_str[100];
  for (size_t i = 0; i < input.size; i++) {
    for (size_t target_num = input.items[i].start;
         target_num <= input.items[i].end; target_num++) {
      sprintf(target_str, "%zu", target_num);
      size_t end = 0;
      while (target_str[end] != '\0') {
        end += 1;
      }
      bool equal = true;
      size_t left = 0;
      size_t middle = end / 2;
      size_t right = end / 2;
      while (left < middle && right < end && equal) {
        equal = target_str[left] == target_str[right];
        left += 1;
        right += 1;
      }
      if (equal && (end - right == 0)) {
        VecPush(invalid_ids, target_num);
      }
    }
  }

  size_t out = 0;
  for (size_t i = 0; i < invalid_ids.size; i++) {
    out += invalid_ids.items[i];
  }
  printf("%zu\n", out);

  VecFree(input);
  VecFree(invalid_ids);
  return EXIT_SUCCESS;
}
