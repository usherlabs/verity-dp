#[cfg(not(any(feature = "private-facets", feature = "public-facets")))]
compile_error!("You must enable at least one feature of 'private-facets' or 'public-facets'");

fn main() {}
