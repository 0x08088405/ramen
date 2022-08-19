use std::borrow::Cow;

#[derive(Debug)]
pub enum Error {
    /// The request can't be completed because of a user error such as an invalid parameter or program state
    Invalid,

    /// The program has run out of stack or a heap allocation has failed
    OutOfMemory,

    /// There system did not have the resources to fulfil the request
    SystemResources,

    /// A human-readable error string was returned by the operating system
    Text(Cow<'static, str>),

    /// The platform does not support the features necessary for this request
    Unsupported,
}
