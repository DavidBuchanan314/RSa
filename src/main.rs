/* I'm just going to stick everything in one file for now so I don't
 * end up bikeshedding over stupid project structure decisions
 */
extern crate hex;

/// This type represents positive arbitrary precision integers
/// The least significant word comes first
#[derive(Debug)]
struct BigInt {
	storage: Vec<u64>
}

impl From<Vec<u8>> for BigInt {
	fn from(data: Vec<u8>) -> BigInt {
		let mut storage = vec![0u64; (data.len()+7)/8];
		let mut si = 0;
		for di in (0..data.len()).rev() {
			storage[si/8] |= (data[di] as u64) << (si%8)*8;
			si += 1;
		}
		BigInt{storage: storage}
	}
}

impl From<BigInt> for Vec<u8> {
	fn from(bigint: BigInt) -> Vec<u8> {
		let mut data = vec![0u8; bigint.storage.len()*8];
		for i in 0..bigint.storage.len() {
			for j in 0..8 {
				data[i*8+j] = (bigint.storage[i] >> (8*j)) as u8;
			}
		}
		data.reverse(); // expensive!!
		return data;
	}
}

fn main() {
	let foo = hex::decode("deadbeefcafebabec001d00d").unwrap();
	let bar = BigInt::from(foo);
	println!("{:?}", bar);
	println!("{:?}", hex::encode(Vec::from(bar)));
}
