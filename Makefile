PROJECT = rust-ps
SOURCE = ./src/main.rs
TARGET = ./target/debug/$(PROJECT)
RELEASE = ./target/release/$(PROJECT)
INPUT = ./input.txt
OUTPUT = ./output.txt
ANALYZE = ./analyze.txt
MEMCHK = ./memory_test/memcheck.txt

BACKTRACE = RUST_BACKTRACE=1

.SILENT: all run $(TARGET) $(RELEASE)

all: $(TARGET)
	$(BACKTRACE) $^ < $(INPUT)

run: $(RELEASE)
	$(BACKTRACE) $^ < $(INPUT)

boj: $(RELEASE)
	cargo run --release --bin=autocheck -- $(id)

at:
	cargo +1.42.0 run < $(INPUT)

perf: $(RELEASE)
	sudo operf $^ < $(INPUT)
	opannotate --source > $(ANALYZE)

memchk: $(TARGET)
	valgrind --leak-check=full --show-leak-kinds=all --log-file=$(MEMCHK) -v --error-limit=no $^ < $(INPUT)

$(TARGET): $(SOURCE)
	cargo build

$(RELEASE): $(SOURCE)
	cargo build --release
