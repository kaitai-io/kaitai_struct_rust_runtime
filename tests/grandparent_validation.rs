use kaitai::*;

#[derive(Default, Debug)]
struct Grandparent<'s> {
    value: &'s [u8],
    parent: Option<Parent<'s>>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for Grandparent<'s> {
    type Root = Self;
    type ParentStack = (KStructUnit);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: TypedStack<Self::ParentStack>,
    ) -> KResult<'s, ()> {
        self.value = _io.read_bytes(1)?;

        let mut tmp = Parent::default();
        tmp.read(_io, Some(self), _parent.push(self));
        self.parent = Some(tmp);
        Ok(())
    }
}

#[derive(Default, Debug)]
struct Parent<'s> {
    value: &'s [u8],
    child: Option<Child>,
}
impl<'r, 's: 'r> KStruct<'r, 's> for Parent<'s> {
    type Root = Grandparent<'s>;
    type ParentStack = (&'r Grandparent<'s>, <Grandparent<'s> as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: TypedStack<Self::ParentStack>,
    ) -> KResult<'s, ()> {
        self.value = _io.read_bytes(1)?;

        let mut tmp = Child::default();
        tmp.read(_io, _root, _parent.push(self));
        self.child = Some(tmp);

        Ok(())
    }
}

#[derive(Default, Debug)]
struct Child {
    gp_value: u8,
}
impl<'r, 's: 'r> KStruct<'r, 's> for Child {
    type Root = Grandparent<'s>;
    type ParentStack = (&'r Parent<'s>, <Parent<'s> as KStruct<'r, 's>>::ParentStack);

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: TypedStack<Self::ParentStack>,
    ) -> KResult<'s, ()> {
        self.gp_value = _parent.pop().peek().value[0];

        Ok(())
    }
}

#[test]
fn basic_parse() {
    let bytes = [0u8, 1];
    let r =  BytesReader::new(&bytes);

    let mut gp = Grandparent::default();
    gp.read(&r, None, KStructUnit::parent_stack());

    assert_eq!(gp.value[0], gp.parent.unwrap().child.unwrap().gp_value);
}