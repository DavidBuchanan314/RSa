/* I'm just going to stick everything in one file for now so I don't
 * end up bikeshedding over stupid project structure decisions
 */
extern crate hex;

use std::cmp::Ordering;

type BigIntWord = u32;
type BigIntDWord = u64;
const WORDBYTES: usize = 4;
const BYTEBITS: usize = 8;
const WORDBITS: usize = WORDBYTES * BYTEBITS;

/// This type represents positive arbitrary precision integers
/// The least significant word comes first
#[derive(Debug, Clone)]
struct BigInt {
	storage: Vec<BigIntWord>
}

impl BigInt {
	fn normalise(&mut self) -> () {
		while self.storage.last() == Some(&0) {
			self.storage.pop();
		}
	}
	
	// helper function to get the ith data word, even if it's off the end
	fn get_word(&self, i: usize) -> BigIntWord {
		if i >= self.storage.len() {
			0
		} else {
			self.storage[i]
		}
	}
	
	fn get_bit(&self, i: usize) -> BigIntWord {
		(self.get_word(i/WORDBITS) >> (i%WORDBITS)) & 1
	}
	
	fn add(&self, other: &BigInt) -> BigInt {
		let length = std::cmp::max(self.storage.len(), other.storage.len());
		let mut result = vec![0; length];
		let mut tmp: BigIntDWord = 0;
		for i in 0..length {
			tmp += self.get_word(i) as BigIntDWord + other.get_word(i) as BigIntDWord;
			result[i] = tmp as BigIntWord;
			tmp >>= WORDBITS; // carry any overflow
		}
		
		if tmp > 0 {
			result.push(tmp as BigIntWord);
		}
		
		return BigInt{storage: result};
	}
	
	fn mul(&self, other: &BigInt) -> BigInt {
		let mut result = BigInt::from(vec![0]);
		for bit in (0..self.storage.len()*WORDBITS).rev() {
			result = result.add(&result); // TODO implement shifts
			if self.get_bit(bit) == 1 {
				result = result.add(&other);
			}
		}
		return result;
	}
}

impl From<Vec<u8>> for BigInt {
	fn from(data: Vec<u8>) -> BigInt {
		let mut storage = vec![0; (data.len()+WORDBYTES-1)/WORDBYTES];
		let mut si = 0;
		for di in (0..data.len()).rev() {
			storage[si/WORDBYTES] |= (data[di] as BigIntWord) << (si%WORDBYTES)*BYTEBITS;
			si += 1;
		}
		let mut result = BigInt{storage: storage};
		result.normalise();
		return result;
	}
}

impl From<BigInt> for Vec<u8> {
	fn from(bigint: BigInt) -> Vec<u8> {
		let mut data = vec![0; bigint.storage.len()*WORDBYTES];
		for i in 0..bigint.storage.len() {
			for j in 0..WORDBYTES {
				data[i*WORDBYTES+j] = (bigint.storage[i] >> (BYTEBITS*j)) as u8;
			}
		}
		while data.last() == Some(&0) {data.pop();}; // strip trailing zeroes
		data.reverse(); // revert to bigendian order
		return data;
	}
}

impl PartialEq for BigInt {
	fn eq(&self, other: &BigInt) -> bool {
		self.storage.eq(&other.storage) // bigints should always be normalised
	}
}

impl PartialOrd for BigInt {
	fn partial_cmp(&self, other: &BigInt) -> Option<Ordering> {
		self.storage.partial_cmp(&other.storage) // it just werks
	}
}

fn main() {
	let foo = BigInt::from(hex::decode("0000000000000000deadbeefcafebabec001d00d").unwrap());
	let bar = BigInt::from(hex::decode("deadbeefcafebabec001d00d").unwrap());
	let baz = BigInt::from(hex::decode("feedfacebadf00d5").unwrap());
	let bat = BigInt::from(hex::decode("bada55c0ffeed00d").unwrap());
	println!("{:?}", foo);
	println!("0x{}", hex::encode(Vec::from(foo.clone())));
	println!("{:?}", foo == bar);
	println!("0x{}", hex::encode(Vec::from(foo.add(&bar))));
	println!("0x{}", hex::encode(Vec::from(baz.mul(&bat))));
}
