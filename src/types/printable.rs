use encoding::{all::ISO_8859_1, DecoderTrap, Encoding};

#[derive(PartialEq, Clone, Hash, Eq)]
pub struct PrintableByteVec(pub Vec<u8>);

impl PrintableByteVec {
    pub fn new(input: Vec<u8>) -> Self {
        PrintableByteVec(input)
    }
}
impl ::std::fmt::Debug for PrintableByteVec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let decoded = ISO_8859_1.decode(&self.0, DecoderTrap::Strict).unwrap();
        write!(f, "{}", decoded)
    }
}

impl ::std::ops::Deref for PrintableByteVec {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(PartialEq, Clone)]
pub struct PrintableByteSlice<'a>(pub &'a [u8]);

impl<'a> PrintableByteSlice<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        PrintableByteSlice(input)
    }
}
impl<'a> ::std::fmt::Debug for PrintableByteSlice<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let decoded = ISO_8859_1.decode(&self.0, DecoderTrap::Strict).unwrap();
        write!(f, "{}", decoded)
    }
}

impl<'a> ::std::ops::Deref for PrintableByteSlice<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
