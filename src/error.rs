#[derive(Debug)]
pub enum Error {
    /// The program has run out of stack or a heap allocation has failed
    OutOfMemory,

    /// TODO: ???
    SystemResources,

    /// A human-readable error string was returned by the operating system
    Other(&'static str),
}
