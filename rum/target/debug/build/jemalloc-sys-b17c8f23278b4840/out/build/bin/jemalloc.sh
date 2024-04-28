#!/bin/sh

prefix=/mnt/h/School/CSC-411/Assignments/Profiling/rum/target/debug/build/jemalloc-sys-b17c8f23278b4840/out
exec_prefix=/mnt/h/School/CSC-411/Assignments/Profiling/rum/target/debug/build/jemalloc-sys-b17c8f23278b4840/out
libdir=${exec_prefix}/lib

LD_PRELOAD=${libdir}/libjemalloc.so.2
export LD_PRELOAD
exec "$@"
