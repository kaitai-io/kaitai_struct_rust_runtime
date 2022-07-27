#!/bin/bash -e

# current script dir
CWD=`pwd`
CSD=`dirname -- "$0"`
CSDF="$CWD/$CSD"

pushd $CSD/../../compiler/jvm/target/universal
# refresh build
unzip -q -o ./kaitai-struct-compiler-0.10-SNAPSHOT.zip

for fullpath in $CSDF/formats/*.ksy; do
    filename="${fullpath##*/}"
	echo generating ${filename%.[^.]*}.rs && ./kaitai-struct-compiler-0.10-SNAPSHOT/bin/kaitai-struct-compiler -t rust --outdir $CSDF/tests $fullpath
done

popd
