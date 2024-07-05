use std::{io::Write, fs::File};
use sp1_sdk::{ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
///
/// This file is generated by running `cargo prove build` inside the `program` directory.
pub const METHOD_ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Setup the inputs.
    let stdin = SP1Stdin::new();

    let client = ProverClient::new();
    let (pk, vk) = client.setup(METHOD_ELF);
    let proof = client.prove_compressed(&pk, stdin).expect("failed to generate proof");

    // Verify the proof.
    client.verify_compressed(&proof, &vk).expect("failed to verify proof");

    // Save proof.
    let proof_data = bincode::serialize(&proof).expect("failed to serialize proof");
    let mut proof_file = File::create("../../sp1.proof").expect("Failed to create sp1 elf file");
    proof_file
        .write_all(&proof_data)
        .expect("failed write sp1 elf to file");

    // Save elf
    let mut elf_file = File::create("../../sp1.elf").expect("Failed to create sp1 elf file");
    elf_file
        .write_all(&METHOD_ELF)
        .expect("failed write sp1 elf to file");
}