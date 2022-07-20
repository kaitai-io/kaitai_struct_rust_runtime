#!/bin/bash -e

# current script dir
CWD=`pwd`
CSD=`dirname -- "$0"`
CSDF="$CWD/$CSD"

find $CSDF/tests ! -name '*_tests.rs' -type f -print -exec rm -f {} +
