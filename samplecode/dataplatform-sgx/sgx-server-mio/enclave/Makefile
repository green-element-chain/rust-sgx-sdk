Rust_Enclave_Name := libenclave.a
Rust_Enclave_Files := $(wildcard src/*.rs)
Rust_Target_Path := $(CURDIR)/../../../../xargo
SGX_DEBUG := 0

ifeq ($(SGX_DEBUG), 1)
	RUST_SGX_APP_FLAG +=
	RUST_SGX_APP_DIR = debug
else
	RUST_SGX_APP_FLAG += --release
	RUST_SGX_APP_DIR = release
endif

.PHONY: all

all: $(Rust_Enclave_Name)

$(Rust_Enclave_Name): $(Rust_Enclave_Files)
ifeq ($(XARGO_SGX), 1)
	RUST_TARGET_PATH=$(Rust_Target_Path) xargo build --target x86_64-unknown-linux-sgx --release
	cp ./target/x86_64-unknown-linux-sgx/$(RUST_SGX_APP_DIR)/libmiora.a ../lib/libenclave.a
else
	cargo build $(RUST_SGX_APP_FLAG)
	cp ./target/$(RUST_SGX_APP_DIR)/libmiora.a ../lib/libenclave.a
endif
