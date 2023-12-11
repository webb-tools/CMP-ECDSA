pub mod dec_q;
pub mod log_star;
pub mod mul;
pub mod mul_star;
pub mod sha2;

pub const SEC_PARAM: usize = 256;
pub const SEC_BYTES: usize = SEC_PARAM / 8;
pub const OT_PARAM: usize = 128;
pub const OT_BYTES: usize = OT_PARAM / 8;
pub const STAT_PARAM: usize = 80;

// ZK_MOD_ITERATIONS is the number of iterations that are performed to prove the
// validity of a Paillier-Blum modulus N.
// Theoretically, the number of iterations corresponds to the statistical
// security parameter, and would be 80.
// The way it is used in the refresh protocol ensures that the prover cannot
// guess in advance the secret ρ used to instantiate the hash function.
// Since sampling primes is expensive, we argue that the security can be
// reduced.
pub const ZK_MOD_ITERATIONS: usize = 12;

#[allow(clippy::identity_op)]
pub const L: usize = 1 * SEC_PARAM; // = 256
pub const L_PRIME: usize = 5 * SEC_PARAM; // = 1280
pub const EPSILON: usize = 2 * SEC_PARAM; // = 512
pub const L_PLUS_EPSILON: usize = L + EPSILON; // = 768
pub const L_PRIME_PLUS_EPSILON: usize = L_PRIME + EPSILON; // = 1792

pub const BITS_INT_MODN: usize = 8 * SEC_PARAM; // = 2048
pub const BYTES_INT_MODN: usize = BITS_INT_MODN / 8; // = 256

pub const BITS_BLUM_PRIME: usize = 4 * SEC_PARAM; // = 1024
pub const BITS_PAILLIER: usize = 2 * BITS_BLUM_PRIME; // = 2048

pub const BYTES_PAILLIER: usize = BITS_PAILLIER / 8; // = 256
pub const BYTES_CIPHERTEXT: usize = 2 * BYTES_PAILLIER; // = 512
