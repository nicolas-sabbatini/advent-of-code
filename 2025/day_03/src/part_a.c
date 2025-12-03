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

Vec(size_t);

int main(int argc, char *argv[argc + 1]) {
  char *file_name = argv[1];
  char buffer[MAX_LINE_LENGTH];
  FILE *fp = fopen(file_name, "r");
  char *sub_string = malloc(sizeof(char) * 2); // 1 char + null terminator
  Vec_size_t input = {};
  size_t out = 0;
  while (fgets(buffer, MAX_LINE_LENGTH, fp) != NULL) {
    for (size_t i = 0; buffer[i] != '\0' && buffer[i] != '\n'; i++) {
      strncpy(sub_string, &buffer[i], 1);
      size_t new_num = strtoull(sub_string, NULL, 10);
      VecPush(input, new_num);
    }
    size_t max_i = 0;
    for (size_t i = 1; i < input.size - 1; i++) {
      if (input.items[i] > input.items[max_i]) {
        max_i = i;
      }
    }
    size_t s_max_i = max_i + 1;
    for (size_t i = max_i + 2; i < input.size; i++) {
      if (input.items[i] > input.items[s_max_i]) {
        s_max_i = i;
      }
    }
    out += input.items[max_i] * 10 + input.items[s_max_i];
    VecFlush(input);
  }
  free(sub_string);
  fclose(fp);
  VecFree(input);

  printf("Res: %zu\n", out);

  return EXIT_SUCCESS;
}
