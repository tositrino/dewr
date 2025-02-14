#
# Makefile for rust projects
#

# project setup
PROJECT_NAME = lab_template
PROJECT_SYSTEM = $(shell uname -s | tr [:upper:] [:lower:])

# system dependent settings
uname_m := $(shell uname -m)
ifeq ($(uname_m),i386)
  PROJECT_ARCH = x32
  LD_ARCH=$(uname_m)
endif
ifeq ($(uname_m),x86_64)
  PROJECT_ARCH  = x64
  LD_ARCH=$(uname_m)
endif
ifeq ($(uname_m),arm64)
  PROJECT_ARCH  = arm64
  LD_ARCH=arm64
endif
ifeq ($(uname_m),aarch64)
  PROJECT_ARCH  = arm64
  LD_ARCH=$(uname_m)
endif
  
# tools
ASM := as
CC := gcc
CFLAGS := -Wall
LD := ld
CUT := cut
MAKE := make
SHELL := /bin/bash
TR := tr
ifeq ($(CUR_SYSTEM),darwin)
  GREP  := ggrep
  SED   := gsed
  AWK   := gawk
  SORT := gsort
else
  GREP  := grep
  SED   := sed
  AWK   := awk
  SORT := sort
endif

.PHONY: help all clean check info init

# targets

help: ## Display this help message
	@echo "[HELP]"
	@$(GREP) -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | $(SORT) | $(AWK) 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

all: ## (re)uild the whole project and run it
	@echo "[ALL]"
	$(MAKE) clean
	$(MAKE) build
	$(MAKE) test
	$(MAKE) doc
	$(MAKE) run

build:  ## Build the project using cargo
	cargo build

bump: ## Bump the version number
	@echo "Current version is $(shell cargo pkgid | cut -d# -f2)"
	@read -p "Enter new version number: " version; \
	updated_version=$$(cargo pkgid |  $(CUT) -d# -f2 | $(SED) -E "s/([0-9]+\.[0-9]+\.[0-9]+)$$/$$version/"); \
	$(SED) -i -E "s/^version = .*/version = \"$$updated_version\"/" Cargo.toml
	@echo "New version is $(shell cargo pkgid | cut -d# -f2)"%

check:  ## Check versions of rust tools
	@echo "rust command-line utility versions:  "
	rustc --version
	cargo --version
	rustfmt --version
	rustup --version
	clippy-driver

clean: ## Clean the project
	@echo "[CLEAN]"
	cargo clean

doc:  ## create project documentation using cargo
	cargo doc

format:  ## Format the project using cargo
	@rustup component add rustfmt 2> /dev/null
	cargo fmt --quiet

info: ## Display project information
	@echo "[INFO]"
	@echo "  PROJECT_ARCH    = [$(PROJECT_ARCH)"]
	@echo "  PROJECT_SYSTEM  = [$(PROJECT_SYSTEM)]"
	@echo "  AWK             = [$(AWK)]"
	@echo "  CUT             = [$(CUT)]"
	@echo "  GREP            = [$(GREP)]"
	@echo "  MAKE            = [$(MAKE)]"
	@echo "  SED             = [$(SED)]"
	@echo "  SHELL           = [$(SHELL)]"
	@echo "  SORT            = [$(SORT)]"
	@echo "  TR              = [$(TR)]"

init: ## Initialize the project
	@echo "[INIT]"

lint: ## Lint the project using cargo
	@rustup component add clippy 2> /dev/null
	cargo clippy --quiet

release: ## Run the project using cargo
	cargo build --release

run: ## Run the project using cargo
	cargo run

test: ## Test the project using cargo
	cargo test  --quiet

