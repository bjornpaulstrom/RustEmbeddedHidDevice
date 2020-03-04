#!/bin/bash
# Exit if anything fails
set -e

# Find out where the pretty printer Python module is
RUSTC_SYSROOT=`rustc --print=sysroot`
GDB_PYTHON_MODULE_DIRECTORY="$RUSTC_SYSROOT/lib/rustlib/etc"

# Run GDB with the additional arguments that load the pretty printers
PYTHONPATH="$PYTHONPATH:$GDB_PYTHON_MODULE_DIRECTORY" arm-none-eabi-gdb
 \
  -d "$GDB_PYTHON_MODULE_DIRECTORY" \
  -iex "add-auto-load-safe-path $GDB_PYTHON_MODULE_DIRECTORY" \
  "$@"
