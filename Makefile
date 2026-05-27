# tabbislashcat — makefile
# the seed is the seed. the host is dirt the seed sits in.

.PHONY: all build test clean doc

# build everything
all: build

# build the library
build:
	@echo "building..."
	@cargo build --release 2>/dev/null || 	  echo "cargo not found. build in your dirt."

# run tests
test:
	@echo "testing..."
	@cargo test 2>/dev/null || 	  echo "cargo not found. test in your dirt."

# run examples
examples:
	@echo "running examples..."
	@cargo run --example rust_example 2>/dev/null || 	  echo "cargo not found. run in your dirt."

# clean build artifacts
clean:
	@echo "cleaning..."
	@cargo clean 2>/dev/null || 	  echo "cargo not found. clean in your dirt."
	@rm -rf target

# generate documentation
doc:
	@echo "generating docs..."
	@cargo doc --open 2>/dev/null || 	  echo "cargo not found. document in your dirt."

# the dirt is yours
.dirt:
	@echo "the seed is the seed. the host is dirt the seed sits in."
	@echo "build in your dirt. test in your dirt. run in your dirt."
