pragma circom 2.1.4;

// include "circomlib/poseidon.circom";
// include "https://github.com/0xPARC/circom-secp256k1/blob/master/circuits/bigint.circom";


// Prove that I know some set of "witness" adders such that starting a
// Fibonacci sequence with some starting public inputs and getting to
// some output public inputs
template Fib () {
    signal input step_in[2];

    signal output step_out[2];

    signal input adder;

    step_out[0] <== step_in[0] + adder;
    step_out[1] <== step_in[0] + step_in[1];
}

component main { public [step_in] } = Fib();

/* INPUT = {
    "initial": {
        "step": [1, 1]
    },
    "witness": [
        {"adder": 1},
        {"adder": 2},
        {"adder": 1}
    ]
} */