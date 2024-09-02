use tiny_keccak::{Hasher, Sha3};
use zk_rust_io;

pub fn main() {
    let answers: String = zk_rust_io::read();
    let mut sha3 = Sha3::v256();
    let mut output = [0u8; 32];

    sha3.update(&answers.as_bytes());

    sha3.finalize(&mut output);

    if output
        != [
            232, 202, 155, 157, 82, 242, 126, 73, 75, 22, 197, 34, 41, 170, 163, 190, 22, 29, 192,
            5, 99, 134, 186, 25, 77, 128, 188, 154, 238, 70, 245, 229,
        ]
    {
        panic!("Answers do not match");
    }
}
