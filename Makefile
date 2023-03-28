## help: Prints this help message
help:
	@echo "\nContacts App\nUsage: \n"
	@sed -n "s/^##//p" ${MAKEFILE_LIST} | column -t -s ":" |  sed -e "s/^/ /"

## build: Compile the current package
build:
	@cargo build

## update: Update the dependencies of the current package
update:
	@cargo build		

# ## run: Run a binary of the local package
# run:
# 	@cargo run

## check: Analyze the current package and report errors, but don't build object files
check:
	@cargo check --verbose

## release: Release the current package
release:
	@cargo build --release

## clean: Clean the current package
clean:
	@rm -rdf target/ Cargo.lock

## fmt: Format all Rust files of the current crate
fmt:
	@cargo fmt -- --emit=files

## test: Run the tests
test:
	@cargo test --verbose

.PHONY: help build update run check release clean fmt test