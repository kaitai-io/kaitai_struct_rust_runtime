//! Example using hand-coded structs to validate that the borrow checker
//! will allow our code to actually run

use kaitai::{BytesReader, KError, KResult, KStream, KStruct, KStructUnit, TypedStack};

#[derive(Debug, PartialEq, Clone, Default)]
struct TestRootStruct {
    pub bytes: u8,
    pub child: Option<TestChildStruct>,
}
#[derive(Debug, PartialEq, Clone, Default)]
struct TestChildStruct {
    pub bytes: u8,
    pub root_bytes: u8,
    pub parent_bytes: u8,
    pub child2: Option<TestChildStruct2>,
}

#[derive(Debug, PartialEq, Clone, Default)]
struct TestChildStruct2 {
    pub bytes: u8,
    pub root_bytes: u8,
    pub parent_bytes: u8,
}

impl<'r, 's: 'r> KStruct<'r, 's> for TestRootStruct {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: TypedStack<Self::ParentStack>,
    ) -> KResult<'s, ()> {
        self.bytes = _io.read_u1()?;

        let mut child = TestChildStruct::default();
        child.read(_io, Some(self), _parent.push(self))?;
        self.child = Some(child);

        Ok(())
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for TestChildStruct {
    type Root = <TestRootStruct as KStruct<'r, 's>>::Root;
    type ParentStack = (&'r TestRootStruct, <TestRootStruct as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: TypedStack<Self::ParentStack>,
    ) -> KResult<'s, ()> {
        self.bytes = _io.read_u1()?;
        _root.map(|r| self.root_bytes = r.bytes).ok_or(KError::MissingRoot)?;
        self.parent_bytes = _parent.peek().bytes;

        let mut child = TestChildStruct2::default();
        child.read(_io, _root, _parent.push(self))?;
        self.child2 = Some(child);

        Ok(())
    }
}

impl<'r, 's: 'r> KStruct<'r, 's> for TestChildStruct2 {
    type Root = <TestRootStruct as KStruct<'r, 's>>::Root;
    type ParentStack = (&'r TestChildStruct, <TestChildStruct as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: TypedStack<Self::ParentStack>,
    ) -> KResult<'s, ()> {
        self.bytes = _io.read_u1()?;
        _root.map(|r| self.root_bytes = r.bytes).ok_or(KError::MissingRoot)?;
        self.parent_bytes = _parent.peek().bytes;

        Ok(())
    }
}

#[test]
fn basic_parse() {
    let bytes = vec![1, 2, 3];
    let mut reader = BytesReader::new(&bytes);

    let mut root = TestRootStruct::default();
    let res = root.read(&mut reader, None, KStructUnit::parent_stack());
    assert!(res.is_ok());

    assert_eq!(1, root.bytes);
    assert!(root.child.is_some());

    let child = root.child.as_ref().unwrap();

    assert_eq!(2, child.bytes);
    assert_eq!(1, child.root_bytes);
    assert_eq!(1, child.parent_bytes);
    assert!(child.child2.is_some());

    let child2 = child.child2.as_ref().unwrap();
    assert_eq!(3, child2.bytes);
    assert_eq!(1, child2.root_bytes);
    assert_eq!(2, child2.parent_bytes);
}
