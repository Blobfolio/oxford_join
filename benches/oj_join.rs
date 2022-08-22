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
	Bench::new("oxford_and([T; 1])")
		.run(|| ONE.oxford_and()),

	Bench::new("oxford_and([T; 2])")
		.run(|| TWO.oxford_and()),

	Bench::new("oxford_and([T; 3])")
		.run(|| THREE.oxford_and()),

	Bench::new("oxford_and([T; 5])")
		.run(|| FIVE.oxford_and()),

	Bench::new("oxford_and(&[T])")
		.run(|| SLICE.oxford_and())
);
