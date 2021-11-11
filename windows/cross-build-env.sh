# MinGW x86_64 cross compiler triple
MINGW_CHOST=x86_64-w64-mingw32

# Export symbol definitions file for python3.dll
PYTHON3_DLL_SYMS=windows/abi3-py37/python3.def

# Set the rustc target to Windows x86_64
export CARGO_BUILD_TARGET=x86_64-pc-windows-gnu

# Set the PyO3 cross import library path
export PYO3_CROSS_LIB_DIR="$(pwd)/target/${CARGO_BUILD_TARGET}/${MINGW_CHOST}/lib"

# Generate the python3.dll import library
# from the export symbol definitions file:

mkdir -p ${PYO3_CROSS_LIB_DIR}

${MINGW_CHOST}-dlltool \
  --input-def ${PYTHON3_DLL_SYMS} \
  --output-lib ${PYO3_CROSS_LIB_DIR}/python3.dll.a
