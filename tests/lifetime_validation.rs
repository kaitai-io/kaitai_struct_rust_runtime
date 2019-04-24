//! Example using hand-coded structs to validate that the borrow checker
//! will allow our code to actually run

/*
use kaitai::{BytesReader, KError, KStream, KStructUnit, KStruct};

#[derive(Debug, PartialEq, Clone, Default)]
struct TestRootStruct<'a> {
    pub bytes: &'a [u8],
    pub child: Option<&'a TestChildStruct<'a>>,
    parent: Option<&'a KStructUnit<'a>>,
    root: Option<&'a TestRootStruct<'a>>,
}
#[derive(Debug, PartialEq, Clone, Default)]
struct TestChildStruct<'a> {
    pub bytes: &'a [u8],
    parent: Option<&'a TestRootStruct<'a>>,
    root: Option<&'a TestRootStruct<'a>>,
}

impl<'a> KStruct<'a> for TestRootStruct<'a> {
    type Parent = KStructUnit<'a>;
    type Root = TestRootStruct<'a>;

    fn new(_parent: Option<&'a Self::Parent>, _root: Option<&'a Self::Root>) -> Self
    where
        Self: Sized {
        TestRootStruct {
            parent: _parent,
            root: _root,
            ..Default::default()
        }
    }

    fn read<'s: 'a, S: KStream>(&mut self, stream: &'s mut S) -> Result<(), KError<'s>> {
        self.bytes = stream.read_bytes(1)?;

        let mut child = TestChildStruct::new(Some(self), Some(self.root()));
        child.read(stream)?;
        self.child = Some(&child);

        Ok(())
    }

    fn root(&self) -> &'a Self::Root {
        self.root.or(Some(self)).unwrap()
    }
}

impl<'a> KStruct<'a> for TestChildStruct<'a> {
    type Parent = TestRootStruct<'a>;
    type Root = TestRootStruct<'a>;

    fn new(_parent: Option<&'a Self::Parent>, _root: Option<&'a Self::Root>) -> Self where
        Self: Sized {
        TestChildStruct {
            parent: _parent,
            root: _root,
            ..Default::default()
        }
    }

    fn read<'s: 'a, S: KStream>(&mut self, stream: &'s mut S) -> Result<(), KError<'s>> {
        self.bytes = stream.read_bytes(1)?;

        Ok(())
    }

    fn root(&self) -> &'a Self::Root {
        self.root.unwrap()
    }
}

#[test]
fn basic_parse() {
    let bytes = [1, 2];
    let mut reader = BytesReader::from(&bytes[..]);

    let mut root = TestRootStruct::new(None, None);
    let res = root.read(&mut reader);
    assert!(res.is_ok());

    assert_eq!([1], root.bytes);
    assert!(root.child.is_some());
    assert_eq!([2], root.child.unwrap().bytes);
}
*/
