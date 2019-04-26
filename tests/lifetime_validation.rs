//! Example using hand-coded structs to validate that the borrow checker
//! will allow our code to actually run

use kaitai::{BytesReader, KError, KResult, KStream, KStruct, KStructUnit};

#[derive(Debug, PartialEq, Clone, Default)]
struct TestRootStruct<'a> {
    pub bytes: &'a [u8],
    pub child: Option<TestChildStruct<'a>>,
}
#[derive(Debug, PartialEq, Clone, Default)]
struct TestChildStruct<'a> {
    pub bytes: &'a [u8],
    pub root_bytes: &'a [u8],
}

impl<'a> KStruct<'a> for TestRootStruct<'a> {
    type Parent = KStructUnit;
    type Root = TestRootStruct<'a>;

    fn read<'s: 'a, S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&Self::Root>,
        _parent: Option<&Self::Parent>,
    ) -> KResult<'s, ()> {
        self.bytes = _io.read_bytes(1)?;

        // TODO: `new` method in KStruct?
        let mut child = TestChildStruct::default();
        // Implementation note: because callers of `read` can't call us as
        // `struct.read(_io, Some(struct), None)`, we have to use the `or`
        // call below to give an immutable copy of ourselves to the child
        child.read(_io, _root.or(Some(self)), Some(self))?;
        self.child = Some(child);

        Ok(())
    }
}

impl<'a> KStruct<'a> for TestChildStruct<'a> {
    type Parent = TestRootStruct<'a>;
    type Root = TestRootStruct<'a>;

    fn read<'s: 'a, S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&Self::Root>,
        _parent: Option<&Self::Parent>,
    ) -> KResult<'s, ()> {
        self.bytes = _io.read_bytes(1).unwrap();
        _root.map(|r| self.root_bytes = r.bytes).ok_or(KError::MissingRoot)?;

        Ok(())
    }
}

#[test]
fn basic_parse() {
    let bytes = vec![1, 2];
    let mut reader = BytesReader::new(&bytes);

    let mut root = TestRootStruct::default();
    let res = root.read(&mut reader, None, None);
    assert!(res.is_ok());

    assert_eq!([1], root.bytes);
    assert!(root.child.is_some());
    assert_eq!([2], root.child.unwrap().bytes);
}
