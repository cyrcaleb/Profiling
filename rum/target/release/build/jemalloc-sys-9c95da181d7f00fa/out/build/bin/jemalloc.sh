#!/bin/sh

prefix=/mnt/h/School/CSC-411/Assignments/Profiling/rum/target/release/build/jemalloc-sys-9c95da181d7f00fa/out
exec_prefix=/mnt/h/School/CSC-411/Assignments/Profiling/rum/target/release/build/jemalloc-sys-9c95da181d7f00fa/out
libdir=${exec_prefix}/lib

LD_PRELOAD=${libdir}/libjemalloc.so.2
export LD_PRELOAD
exec "$@"
