# Define the Rust target. Xargo will create an ar archive file with the
# following name in the target folder
RUST_TARGET := lib$(KERNEL_MODULE).a

# Enumerate the object files that the C files will compile to
C_OBJECTS := $(patsubst %.c,%.o,$(C_FILES))

# Tell kbuild which files to build
obj-m := $(KERNEL_MODULE).o

# Tell kbuild where the source files are
src := $(BASE_DIR)

# The kernel module will be linked from the C object files and the Rust archive
# The order is important: C objects must come first
$(KERNEL_MODULE)-objs := $(C_OBJECTS) $(RUST_TARGET)

# Strip unused symbols from the input object file
EXTRA_LDFLAGS += --gc-sections --entry=init_module --undefined=cleanup_module
EXTRA_LDFLAGS += $(if $(RELEASE),--strip-all)

# Fix file paths (since this script will be run from the kbuild's working directory)
C_FILES    := $(foreach filepath,$(C_FILES)   ,$(BASE_DIR)/$(filepath))
RUST_FILES := $(foreach filepath,$(RUST_FILES),$(BASE_DIR)/$(filepath))
LLVM_TARGET_SPEC := $(foreach filepath,$(LLVM_TARGET_SPEC),$(BASE_DIR)/$(filepath))

# Determine target directory of cargo's module build
CARGO_BUILD_DIR := $(BASE_DIR)/target/$(UTS_MACHINE)-unknown-none-gnu/$(if $(RELEASE),release,debug)

$(obj)/$(RUST_TARGET): $(RUST_FILES) $(LLVM_TARGET_SPEC) $(BASE_DIR)/$(KBUILD)
# We set RUST_TARGET_PATH because of a bug in cargo/xargo/rustc: the --target flag is relative to the current working
# directory but subsequent invokations of cargo/xargo/rustc might change their working directory. Setting
# RUST_TARGET_PATH ensures that the compiler can find the LLVM target specification.
# We also have to `cd` into $(BASE_DIR) since we are currently in the kernel headers directory.
	(cd $(BASE_DIR); env RUST_TARGET_PATH=$(BASE_DIR) $(CARGO) xbuild $(if $(RELEASE),--release) $(if $(VERBOSE),--verbose) --target $(UTS_MACHINE)-unknown-none-gnu)
# After the archive is compiled, copy it to the build directory
	cp "$(CARGO_BUILD_DIR)/$(RUST_TARGET)" $(obj)

$(obj)/%.c : $(BASE_DIR)/%.c $(BASE_DIR)/$(KBUILD)
# KBUILD_CFLAGS is automatically generated by kbuild
	$(CC) $(KBUILD_CFLAGS) -c $(BASE_DIR)/$*.c -o $*.o
