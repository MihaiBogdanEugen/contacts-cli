## help: Prints this help message
help:
	@echo "\nContacts App\nUsage: \n"
	@sed -n "s/^##//p" ${MAKEFILE_LIST} | column -t -s ":" |  sed -e "s/^/ /"

## build: Compile the current package
build:
	@cargo build

## update: Update the dependencies of the current package
update:
	@cargo update		

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
	@cargo fmt

## test: Run the tests
test:
	@cargo test --verbose

## clippy: Run cargo clippy for static ckecks
clippy:
	@cargo clippy --all-targets --all-features --verbose

## start-db: Run docker-compose to start the Postgres db
start-db:
	@docker-compose up -d

## stop-db: Run docker-compose to stop the Postgres db
stop-db:
	@docker-compose down

.PHONY: help build update run check release clean fmt test clippy start-db stop-db