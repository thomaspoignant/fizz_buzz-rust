COLOR ?= auto # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)
BINARY_NAME=rust-fizzbuzz-rest-api

all: build

build:
	@$(CARGO) build

check:
	@$(CARGO) check

clean:
	@$(CARGO) clean

install: build
	@$(CARGO) install

run: build
	@$(CARGO) run

test: build
	@$(CARGO) test

release:
    @$(CARGO) build --release

docker-build: clean build
	docker build -t $(BINARY_NAME) .

docker-run: docker-build
	docker run --rm -p 8080:8080 $(BINARY_NAME):latest
