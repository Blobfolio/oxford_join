/*!
# Benchmark: Oxford Join
*/

use brunch::{
	Bench,
	benches,
};
use oxford_join::{
	join,
	oxford_join,
};
use std::range::legacy::RangeInclusive;



const ONE: [&str; 1] = ["Apples"];
const TWO: [&str; 2] = ["Apples", "Bananas"];
const THREE: [&str; 3] = ["Apples", "Bananas", "Oranges"];
const U8: RangeInclusive<u8> = u8::MIN..=u8::MAX;


benches!(
	Bench::new("join(ONE)").run(|| join!(ONE.iter()).to_string()),
	Bench::new("join(TWO)").run(|| join!(TWO.iter()).to_string()),
	Bench::new("join(THREE)").run(|| join!(THREE.iter()).to_string()),
	Bench::new("join(u8::MIN..=u8::MAX)").run(|| join!(U8).to_string()),

	Bench::spacer(),

	Bench::new("join(\", \", ONE)").run(|| join!(", ", ONE.iter()).to_string()),
	Bench::new("join(\", \", TWO)").run(|| join!(", ", TWO.iter()).to_string()),
	Bench::new("join(\", \", THREE)").run(|| join!(", ", THREE.iter()).to_string()),
	Bench::new("join(\", \", u8::MIN..=u8::MAX)").run(|| join!(", ", U8).to_string()),

	Bench::spacer(),

	Bench::new("oxford_join(ONE)").run(|| oxford_join!(ONE.iter()).to_string()),
	Bench::new("oxford_join(TWO)").run(|| oxford_join!(TWO.iter()).to_string()),
	Bench::new("oxford_join(THREE)").run(|| oxford_join!(THREE.iter()).to_string()),
	Bench::new("oxford_join(u8::MIN..=u8::MAX)").run(|| oxford_join!(U8).to_string()),
);
