use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub enum Material {}

#[enum_dispatch(Material)]
pub trait Reflect {}
