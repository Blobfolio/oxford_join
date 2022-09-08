/*!
# Benchmark: Oxford Join
*/

use brunch::{
	Bench,
	benches,
};
use oxford_join::OxfordJoin;



const ONE: [&str; 1] = ["Apples"];
const TWO: [&str; 2] = ["Apples", "Bananas"];
const THREE: [&str; 3] = ["Apples", "Bananas", "Oranges"];
const FIVE: [&str; 5] = ["Apples", "Bananas", "Oranges", "Pears", "Jackfruit"];
const SLICE: &[&str] = &["Apples", "Bananas", "Oranges", "Pears", "Jackfruit"];



benches!(
	Bench::new("<[T; 1]>::oxford_and()")
		.run(|| ONE.oxford_and()),

	Bench::new("<[T; 2]>::oxford_and()")
		.run(|| TWO.oxford_and()),

	Bench::new("<[T; 3]>::oxford_and()")
		.run(|| THREE.oxford_and()),

	Bench::new("<[T; 5]>::oxford_and()")
		.run(|| FIVE.oxford_and()),

	Bench::spacer(),

	Bench::new("<&[T]>::oxford_and()")
		.run(|| SLICE.oxford_and()),
);
