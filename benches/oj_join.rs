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
	Bench::new("oxford_and", "([T; 1])")
		.with(|| ONE.oxford_and()),

	Bench::new("oxford_and", "([T; 2])")
		.with(|| TWO.oxford_and()),

	Bench::new("oxford_and", "([T; 3])")
		.with(|| THREE.oxford_and()),

	Bench::new("oxford_and", "([T; 5])")
		.with(|| FIVE.oxford_and()),

	Bench::new("oxford_and", "(&[T])")
		.with(|| SLICE.oxford_and())
);
