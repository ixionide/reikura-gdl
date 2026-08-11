mod deobfuscator;
pub mod filters;

pub use deobfuscator::Deobfuscator;
pub const SIGNATURE: &[u8] = b"SECRETFILTER100a";
