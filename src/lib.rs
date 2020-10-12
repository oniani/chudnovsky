use rug::{ops::Pow, Float};

/// The Chudnovsky algorithm for calculating the digits of Pi
pub fn pi(prec: u32, its: u32) -> Float {
    // Define the constants
    const A:   u32 = 42_688_0;
    const B:   u32 = 10_005;
    const K:   u32 = 6;
    const K_S: u32 = 12;
    const L:   u32 = 135_914_09;
    const L_S: u32 = 545_140_134;
    const M:   u32 = 1;
    const X:   u32 = 1;
    const X_S: i64 = -262_537_412_640_768_000;
    const S:   u32 = 0;

    // Initialize the values
    let a     = Float::with_val(prec, A);
    let b     = Float::with_val(prec, B).sqrt();
    let c     = Float::with_val(prec, &a * &b);
    let mut k = Float::with_val(prec, K);
    let k_s   = Float::with_val(prec, K_S);
    let mut l = Float::with_val(prec, L);
    let l_s   = Float::with_val(prec, L_S);
    let mut m = Float::with_val(prec, M);
    let mut x = Float::with_val(prec, X);
    let x_s   = Float::with_val(prec, X_S);
    let mut s = Float::with_val(prec, S);

    // Iterate
    for q in 0..its {
        // S = S + M_q * L_q / X_q
        s += Float::with_val(prec, &m * &l) / &x;

        // M_q = M_q * (K_q^3 - 16 * K_q) / (q + 1)^3
        m *= (k.clone().pow(3) - 16 * k.clone())
            / Float::with_val(prec, (q + 1).pow(3));

        // Update K_q, L_q, and X_q
        k += &k_s;
        l += &l_s;
        x *= &x_s;
    }

    // C * (1 / S)
    s = Float::with_val(prec, 1) / &s;

    // Pi
    let pi = Float::with_val(prec, &c) * &s;

    pi
}
