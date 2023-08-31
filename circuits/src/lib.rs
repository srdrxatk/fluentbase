#![allow(
    dead_code,
    unreachable_patterns,
    unused_macros,
    clippy::too_many_arguments,
    clippy::type_complexity
)]
#![deny(unsafe_code)]

extern crate core;

mod constraint_builder;
mod fluentbase_circuit;
mod gadgets;
mod pi_circuit;
mod poseidon_circuit;
mod prover;
mod runtime_circuit;
mod rwasm_circuit;
mod state_circuit;
#[cfg(test)]
mod testing;
mod trace_step;
mod unrolled_bytecode;
mod util;
