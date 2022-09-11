/*!
# Benchmark: Oxford Join
*/

use brunch::{
	Bench,
	benches,
};
use oxford_join::OxfordJoin;
use std::collections::{
	BTreeMap,
	BTreeSet,
};



const ONE: [&str; 1] = ["Apples"];
const TWO: [&str; 2] = ["Apples", "Bananas"];
const THREE: [&str; 3] = ["Apples", "Bananas", "Oranges"];
const FIVE: [&str; 5] = ["Apples", "Bananas", "Oranges", "Pears", "Jackfruit"];
const THIRTYTWO: [&str; 32] = [
	"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
	"G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V",
];
const SLICE: &[&str] = &["Apples", "Bananas", "Oranges", "Pears", "Jackfruit"];



fn main() {
	let map = FIVE.into_iter().enumerate().collect::<BTreeMap<usize, &str>>();
	let set = BTreeSet::from(FIVE);


	benches!(
		inline:

		Bench::new("<[T; 1]>::oxford_and()").run(|| ONE.oxford_and()),
		Bench::new("<[T; 2]>::oxford_and()").run(|| TWO.oxford_and()),
		Bench::new("<[T; 3]>::oxford_and()").run(|| THREE.oxford_and()),
		Bench::new("<[T; 5]>::oxford_and()").run(|| FIVE.oxford_and()),
		Bench::new("<[T; 32]>::oxford_and()").run(|| THIRTYTWO.oxford_and()),
		Bench::new("<[T; 32]>::join()").run(|| THIRTYTWO.join(", ")),

		Bench::spacer(),

		Bench::new("<&[T]>::oxford_and()").run(|| SLICE.oxford_and()),

		Bench::spacer(),

		Bench::new("BTreeMap::<_, T>::oxford_and()").run(|| map.oxford_and()),
		Bench::new("BTreeSet::<T>::oxford_and()").run(|| set.oxford_and()),
	);
}
