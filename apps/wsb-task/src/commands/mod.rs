mod cargo;
pub use cargo::build_pilot;
pub use cargo::cargo_build;

pub struct Action<A>(pub A);
