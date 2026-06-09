mod _const;
mod _param;
mod _trait;

mod ed;
pub use ed::Ed;
mod ls;
pub use ls::Ls;
mod lsbs;
pub use lsbs::Lsbs;
mod sret;
pub use sret::Sret;
mod jp;
pub use jp::Jp;
mod js;
pub use js::Js;
mod rt;
pub use rt::Rt;
mod onjp;
pub use onjp::Onjp;
mod onjs;
pub use onjs::Onjs;

pub use _const::INSTRUCTIONS;
pub use _param::*;
pub use _trait::{Evaluate, Instruction, Parameters, ReadParam};
