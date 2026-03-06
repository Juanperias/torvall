MAKEFLAGS += -rR
.SUFFIXES:

override DIST := kernel
override USER_VARIABLE = $(if $(filter $(origin $(1)),default undefined),$(eval override $(1) := $(2)))

$(call USER_VARIABLE,ARCH,x86_64)


ifeq ($(RUST_TARGET),)
    override RUST_TARGET := torvall-x86_64-target
endif

ifeq ($(RUST_PROFILE),)
    override RUST_PROFILE := dev
endif

override RUST_PROFILE_SUBDIR := $(RUST_PROFILE)
ifeq ($(RUST_PROFILE),dev)
    override RUST_PROFILE_SUBDIR := debug
endif

.PHONY: all
all:
	RUSTFLAGS="-C relocation-model=static -C code-model=large" cargo build --target $(RUST_TARGET).json --profile $(RUST_PROFILE) -v
	cp target/$(RUST_TARGET)/$(RUST_PROFILE_SUBDIR)/$$(cd target/$(RUST_TARGET)/$(RUST_PROFILE_SUBDIR) && find -maxdepth 1 -perm -111 -type f) kernel

.PHONY: clean
clean:
	cargo clean
	rm -rf kernel

