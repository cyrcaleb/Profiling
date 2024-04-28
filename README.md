# Identifies you and your programming partner by name
- Caleb Cyr & Rafael Mendez

# Acknowledges help you may have received from or collaborative work you may have undertaken with others
- All of the code and work for this Assignment was done by us, we did not take in any work from any outside sources.

# Explains what routine in the final um takes up the most time, and says whether the assembly code could be improved and how
- The routine that takes up the most time in our program according to Samply and their flame graph is `alloc::raw_vec::RawVec::allocate_in` and deals with allocating space for vectors. This occurs most frequently in our `mapseg` function which corresponds to the opcode that deals with the mapping of segments. Although we used the lto optimizer in our code and to view this assembly via [Godbolt](https://godbolt.org/), we still found a few places where the assembly could be optimized. Specifically in the `alloc::raw_vec::RawVec<T,A>::reserve_for_push::he093ec77719e62f2` portions. This portion of assembly is responsible for expanding the capacity of a raw vector. We found two optimization strategies that could potentially improve the code efficiency:

1. The first thing we noticed is that the logic involving `rcx = rax + rax` and `rsi = 5` is repeated numerous times. This calculation could be computed ahead of time and stored in a temporary register to avoid redundancy. We believe this would benefit the program a fair bit just due to the impact this code segment has on our total run time and the number of times those two pieces of logic are redundantly called.

2. Another thing we noticed is that there are a bunch of conditional jumps based on the value of `r14`. We might be able to actually consolidate or re-order these jumps to improve branch prediction by the CPU.

# Says approximately how many hours you have spent analyzing the problems posed in the assignment
- 1 hour

# Says approximately how many hours you have spent solving the problems after your analysis
- 10 hours
