pub(crate) struct OciProxy {
    _base_address: String,
}

impl OciProxy {
    /// Creates a new `OciProxy` instance.
    pub fn new(base_address: impl Into<String>) -> OciProxy {
        OciProxy {
            _base_address: base_address.into(),
        }
    }
}
