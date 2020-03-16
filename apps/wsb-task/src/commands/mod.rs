mod can_define;
pub use can_define::CanDefine2;

mod cargo;
pub use cargo::build_pilot;
pub use cargo::cargo_build;

pub mod support;

pub struct Action<A>(pub A);
