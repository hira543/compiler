SHELL := /bin/bash
.PHONY: all clean run run-log all-log

ASSEMBLER=nasm
CC=gcc
SOURCE=output.asm
OBJECT=output.o
EXECUTABLE=a
FILES=add.sim binop.sim function.sim if.sim print.sim while.sim

RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
NO_COLOR="\033[0m"

all: $(FILES)

$(FILES):
	@echo -e $(GREEN)"=== Starting build for $@ ==="$(NO_COLOR)
	@echo -e $(YELLOW)"Running Cargo for $@..."$(NO_COLOR)
	-@cargo run examples/$@ 2>&1 | grep --color=never -E "warning|error" || true
	@echo -e $(YELLOW)"Assembling $@..."$(NO_COLOR)
	@$(ASSEMBLER) -f elf64 -o $(OBJECT) $(SOURCE)
	@echo -e $(YELLOW)"Compiling $@..."$(NO_COLOR)
	@$(CC) -g -no-pie -nostartfiles -o $(EXECUTABLE) $(OBJECT)
	@echo -e $(YELLOW)"Executing $@..."$(NO_COLOR)
	@./$(EXECUTABLE)
	@echo -e $(GREEN)"=== Finished build for $@ ==="$(NO_COLOR)

clean:
	@echo -e $(RED)"Cleaning up..."$(NO_COLOR)
	@rm -f $(OBJECT) $(EXECUTABLE)
	@echo -e $(GREEN)"Clean completed."$(NO_COLOR)

# 特定のファイルのみ実行
run-%:
	@echo -e $(GREEN)"=== Running Cargo for $* ==="$(NO_COLOR)
	-@cargo run examples/$* 2>&1 | grep --color=never -E "warning|error" || true
	@echo -e $(YELLOW)"Assembling $*..."$(NO_COLOR)
	@$(ASSEMBLER) -f elf64 -o $(OBJECT) $(SOURCE)
	@echo -e $(YELLOW)"Compiling $*..."$(NO_COLOR)
	@$(CC) -g -no-pie -nostartfiles -o $(EXECUTABLE) $(OBJECT)
	@echo -e $(YELLOW)"Executing $*..."$(NO_COLOR)
	@./$(EXECUTABLE)
	@echo -e $(GREEN)"=== Finished run for $* ==="$(NO_COLOR)

# ログ付きで特定のファイルを実行
run-log-%:
	@echo -e $(GREEN)"=== Running Cargo for $* with logs ==="$(NO_COLOR)
	-@cargo run examples/$* 2>&1
	@echo -e $(YELLOW)"Assembling $*..."$(NO_COLOR)
	@$(ASSEMBLER) -f elf64 -o $(OBJECT) $(SOURCE)
	@echo -e $(YELLOW)"Compiling $*..."$(NO_COLOR)
	@$(CC) -g -no-pie -nostartfiles -o $(EXECUTABLE) $(OBJECT)
	@echo -e $(YELLOW)"Executing $*..."$(NO_COLOR)
	@./$(EXECUTABLE)
	@echo -e $(GREEN)"=== Finished run for $* with logs ==="$(NO_COLOR)

# すべてのファイルをログ付きで実行
all-log:
	@for file in $(FILES); do \
		make run-log-$$file; \
	done

build-asm:
	@echo -e $(YELLOW)"Building assembly..."$(NO_COLOR)
	@$(ASSEMBLER) -f elf64 -o $(OBJECT) $(SOURCE)
	@echo -e $(YELLOW)"Compiling executable..."$(NO_COLOR)
	@$(CC) -g -no-pie -nostartfiles -o $(EXECUTABLE) $(OBJECT)
	@echo -e $(YELLOW)"Running executable..."$(NO_COLOR)
	@./$(EXECUTABLE)

# アセンブリコードを表示しながら特定のファイルを実行
run-with-asm-%:
	@echo -e $(GREEN)"=== Running Cargo for $* ==="$(NO_COLOR)
	-@cargo run examples/$* 2>&1 | grep --color=never -E "warning|error" || true
	@echo -e $(YELLOW)"=== Assembly code for $* ==="$(NO_COLOR)
	@cat $(SOURCE)
	@echo -e $(YELLOW)"Assembling $*..."$(NO_COLOR)
	@$(ASSEMBLER) -f elf64 -o $(OBJECT) $(SOURCE)
	@echo -e $(YELLOW)"Compiling $*..."$(NO_COLOR)
	@$(CC) -g -no-pie -nostartfiles -o $(EXECUTABLE) $(OBJECT)
	@echo -e $(YELLOW)"Executing $*..."$(NO_COLOR)
	@./$(EXECUTABLE)
	@echo -e $(GREEN)"=== Finished run for $* ==="$(NO_COLOR)

# すべてのファイルをアセンブリコードと共に実行
all-with-asm:
	@for file in $(FILES); do \
		make run-with-asm-$$file; \
	done
