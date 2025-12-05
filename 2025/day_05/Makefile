CC = gcc
CFLAGS = -Wall -Wextra -std=c23 -pedantic -lm

RELEASE ?= 0
ifeq ($(RELEASE), 0)
    CFLAGS += -g -O0 -fsanitize=address -fno-omit-frame-pointer -DDEBUG -D_DEBUG
else
    CFLAGS += -O3
endif

compile_flags.txt:
	@echo $(CFLAGS) | tr ' ' '\n' > compile_flags.txt
	@echo $(LDFLAGS) | tr ' ' '\n' >> compile_flags.txt
	@echo $(DFLAGS) | tr ' ' '\n' >> compile_flags.txt

.PHONY: clean compile_flags.txt

bin/part_a: src/part_a.c
	@mkdir -p bin
	$(CC) $(CFLAGS)  $^ -o $@

run_a: bin/part_a
	./$^ input.txt

test_a: bin/part_a
	./$^ test.txt

bin/part_b: src/part_b.c
	@mkdir -p bin
	$(CC) $(CFLAGS)  $^ -o $@

run_b: bin/part_b
	./$^ input.txt

test_b: bin/part_b
	./$^ test.txt

clean:
	rm -fr bin compile_flags.txt
