pub mod asynchronous;
pub mod synchronous;

/// Represents the length of a message body in bytes.
type HeaderType = u32;
///  The size in bytes of the type used to represent the length of a message body.
const HEADER_SIZE: usize = core::mem::size_of::<HeaderType>();

