/* I'm just going to stick everything in one file for now so I don't
 * end up bikeshedding over stupid project structure decisions
 */
extern crate hex;

/// This type represents positive arbitrary precision integers
/// The least significant word comes first
#[derive(Debug, Clone)]
struct BigInt {
	storage: Vec<u64>
}

impl BigInt {
	fn normalise(&mut self) -> () {
		while self.storage.last() == Some(&0) {
			self.storage.pop();
		}
	}
}

impl From<Vec<u8>> for BigInt {
	fn from(data: Vec<u8>) -> BigInt {
		let mut storage = vec![0u64; (data.len()+7)/8];
		let mut si = 0;
		for di in (0..data.len()).rev() {
			storage[si/8] |= (data[di] as u64) << (si%8)*8;
			si += 1;
		}
		let mut result = BigInt{storage: storage};
		result.normalise();
		return result;
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
		while data.last() == Some(&0) {data.pop();}; // strip trailing zeroes
		data.reverse(); // revert to bigendian order
		return data;
	}
}

impl PartialEq for BigInt {
	fn eq(&self, other: &BigInt) -> bool {
		self.storage == other.storage // bigints should always be normalised
	}
}

impl Eq for BigInt {}

fn main() {
	let foo = BigInt::from(hex::decode("0000000000000000deadbeefcafebabec001d00d").unwrap());
	let bar = BigInt::from(hex::decode("deadbeefcafebabec001d00d").unwrap());
	println!("{:?}", foo);
	println!("{}", hex::encode(Vec::from(foo.clone())));
	println!("{:?}", foo == bar);
}
