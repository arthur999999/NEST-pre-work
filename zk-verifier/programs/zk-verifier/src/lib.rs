use anchor_lang::prelude::*;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use groth16_solana::groth16::Groth16Verifier;
use verifying_key::VERIFYINGKEY;

declare_id!("EmQ5kCMotSG7dQWh1bZ47gseHmzXy8sgM6zXMDoMT4cZ");

pub mod verifying_key;
type G1 = ark_bn254::g1::G1Affine;
#[program]
pub mod zk_verifier {

    use super::*;

    pub fn verify(_ctx: Context<Verifier>, output: u64, proof: [u8; 256]) -> Result<()> {
        let proof_a: G1 = <G1 as CanonicalDeserialize>::deserialize_uncompressed(
            &*[&change_endianness(&proof[0..64])[..], &[0u8][..]].concat(),
        )
        .unwrap();

        let mut proof_a_negation = [0u8; 65]; //This negates the point proof_a on the elliptic curve. In elliptic curve cryptography, negating a point P means finding the point -P which lies on the curve.

        <G1 as CanonicalSerialize>::serialize_uncompressed(&-proof_a, &mut proof_a_negation[..])
            .unwrap();
        let proof_a: [u8; 64] = change_endianness(&proof_a_negation[..64])
            .try_into()
            .unwrap();
        let proof_b: [u8; 128] = proof[64..192].try_into().unwrap();
        let proof_c: [u8; 64] = proof[192..256].try_into().unwrap();

        let mut result = [0u8; 32];
        let value_bytes = output.to_be_bytes();
        result[24..].copy_from_slice(&value_bytes);

        let public_inputs = [result];

        let mut verifier =
            Groth16Verifier::new(&proof_a, &proof_b, &proof_c, &public_inputs, &VERIFYINGKEY)
                .unwrap();
        verifier.verify().unwrap();

        Ok(())
    }
}

fn change_endianness(bytes: &[u8]) -> Vec<u8> {
    let mut vec = Vec::new();
    for b in bytes.chunks(32) {
        for byte in b.iter().rev() {
            vec.push(*byte);
        }
    }
    vec
}

#[derive(Accounts)]
pub struct Verifier {}
