COMPILER := gcc
COMPILER_FLAGS := -std=c99 -Wall -Werror -Wextra
COMPILER_LINK = -lm
EXECUTABLES := main

OUTPUT_DIRECTORY := ./bin
TEMPORARY_DIRECTORY := ./obj
SOURCE_DIRECTORY := .

CLANG_FORMAT := /opt/homebrew/bin/clang-format
CLANG_FORMAT_CONFIGURATION := /Users/george/.format/.clang-format

all: ${OUTPUT_DIRECTORY}/${EXECUTABLES}

${TEMPORARY_DIRECTORY}/%.o: ${SOURCE_DIRECTORY}/%.c
	mkdir -p ${TEMPORARY_DIRECTORY}
	$(COMPILER) $(COMPILER_FLAGS) -c $< -o $@

${OUTPUT_DIRECTORY}/main: ${TEMPORARY_DIRECTORY}/main.o
	mkdir -p ${OUTPUT_DIRECTORY}
	$(COMPILER) $^ $(COMPILER_LINK) -o $@

clean:
	rm -r -f ${TEMPORARY_DIRECTORY}
	rm -f ${EXECUTABLES}
	rm -r -f out
	rm -f .src_files

format:
	find . \( -name "*.c" -o -name "*.h" \) > .src_files
	$(CLANG_FORMAT) -i -style='file:$(CLANG_FORMAT_CONFIGURATION)' -files=.src_files

submission:
	mkdir -p out
	zip out/submission.zip *.c *.h makefile