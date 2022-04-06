/*!
# Benchmark: Oxford Join
*/

use brunch::{
	Bench,
	benches,
};
use oxford_join::OxfordJoin;
use std::time::Duration;



const ONE: [&str; 1] = ["Apples"];
const TWO: [&str; 2] = ["Apples", "Bananas"];
const THREE: [&str; 3] = ["Apples", "Bananas", "Oranges"];
const FIVE: [&str; 5] = ["Apples", "Bananas", "Oranges", "Pears", "Jackfruit"];
const SLICE: &[&str] = &["Apples", "Bananas", "Oranges", "Pears", "Jackfruit"];



benches!(
	Bench::new("oxford_and", "([T; 1])")
		.timed(Duration::from_secs(1))
		.with(|| ONE.oxford_and()),

	Bench::new("oxford_and", "([T; 2])")
		.timed(Duration::from_secs(1))
		.with(|| TWO.oxford_and()),

	Bench::new("oxford_and", "([T; 3])")
		.timed(Duration::from_secs(1))
		.with(|| THREE.oxford_and()),

	Bench::new("oxford_and", "([T; 5])")
		.timed(Duration::from_secs(1))
		.with(|| FIVE.oxford_and()),

	Bench::new("oxford_and", "(&[T])")
		.timed(Duration::from_secs(1))
		.with(|| SLICE.oxford_and())
);
