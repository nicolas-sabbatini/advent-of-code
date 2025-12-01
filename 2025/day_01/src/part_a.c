#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#define MAX_LINE_LENGTH 1024

#define Vec(T)                                                                 \
  typedef struct {                                                             \
    T *items;                                                                  \
    size_t size;                                                               \
    size_t capacity;                                                           \
  } Vec_##T
#define VecCreate(T) (Vec_##T){.items = NULL, .size = 0, .capacity = 0}
#define VecPrint(vec, format)                                                  \
  do {                                                                         \
    printf("[ ");                                                              \
    for (size_t i = 0; i < vec.size; i++) {                                    \
      printf(format, vec.items[i]);                                            \
    }                                                                          \
    printf(" ]\n");                                                            \
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

Vec(int64_t);

int main(int argc, char *argv[argc + 1]) {
  Vec_int64_t input = VecCreate(int64_t);
  char *file_name = argv[1];
  char buffer[MAX_LINE_LENGTH];
  FILE *fp = fopen(file_name, "r");
  while (fgets(buffer, MAX_LINE_LENGTH, fp) != NULL) {
    int64_t new_input;
    switch (buffer[0]) {
    case 'L':
      new_input = -1 * atoi(&buffer[1]);
      break;
    case 'R':
      new_input = atoi(&buffer[1]);
      break;
    default:
      exit(EXIT_FAILURE);
    }
    VecPush(input, new_input);
  }

  int64_t arrow = 50;
  int64_t pass = 0;
  for (size_t i = 0; i < input.size; i++) {
    arrow = (arrow + input.items[i]) % 100;
    if (arrow == 0) {
      pass += 1;
    }
  }
  printf("Pass: %li\n", pass);

  fclose(fp);
  VecFree(input);
  return EXIT_SUCCESS;
}
