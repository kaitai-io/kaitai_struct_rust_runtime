//! Example using hand-coded structs to validate that the borrow checker
//! will allow our code to actually run

use kaitai::{BytesReader, KError, KResult, KStream, KStruct, KStructUnit, TypedStack};

#[derive(Debug, PartialEq, Clone, Default)]
struct TestRootStruct<'s> {
    pub bytes: &'s [u8],
    pub child: Option<TestChildStruct<'s>>,
}
#[derive(Debug, PartialEq, Clone, Default)]
struct TestChildStruct<'s> {
    pub bytes: &'s [u8],
    pub root_bytes: &'s [u8],
}

impl<'r, 's: 'r> KStruct<'r, 's> for TestRootStruct<'s> {
    type Root = Self;
    type ParentStack = (KStructUnit);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: TypedStack<Self::ParentStack>,
    ) -> KResult<'s, ()> {
        self.bytes = _io.read_bytes(1)?;

        let mut child = TestChildStruct::default();
        child.read(_io, Some(self), _parent.push(self))?;
        self.child = Some(child);

        Ok(())
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for TestChildStruct<'s> {
    type Root = <TestRootStruct<'s> as KStruct<'r, 's>>::Root;
    type ParentStack = (&'r TestRootStruct<'s>, <TestRootStruct<'s> as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: TypedStack<Self::ParentStack>,
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
    let res = root.read(&mut reader, None, KStructUnit::parent_stack());
    assert!(res.is_ok());

    assert_eq!([1], root.bytes);
    assert!(root.child.is_some());
    assert_eq!([2], root.child.unwrap().bytes);
}
