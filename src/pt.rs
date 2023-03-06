use std::{
    cell::{RefCell, UnsafeCell},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    rc::Rc,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Ref<'a, T> {
    target: NonNull<Option<T>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: Default> Deref for Ref<'a, T> {
    type Target = Option<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { self.target.as_ref() }
    }
}

pub struct RefMut<'a, T> {
    target: NonNull<Option<T>>,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T: Default> Deref for RefMut<'a, T> {
    type Target = Option<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { self.target.as_ref() }
    }
}

impl<'a, T: Default> DerefMut for RefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.target.as_mut() }
    }
}

#[derive(Default, Debug)]
pub struct ParamType<T: Default>(UnsafeCell<Option<T>>);

impl<T: Default> ParamType<T> {
    pub fn new(t: T) -> Self {
        Self(UnsafeCell::new(Some(t)))
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        let value = unsafe { NonNull::new_unchecked(self.0.get()) };
        Ref {
            target: value,
            _marker: PhantomData,
        }
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        let value = unsafe { NonNull::new_unchecked(self.0.get()) };
        RefMut {
            target: value,
            _marker: PhantomData,
        }
    }
}

impl<T: Default> Deref for ParamType<T> {
    type Target = Option<T>;
    fn deref<'a>(&'a self) -> &'a Self::Target {
        unsafe { &*self.0.get() }
    }
}

impl<T: Default> DerefMut for ParamType<T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Self::Target {
        self.0.get_mut()
    }
}

impl<T: Default> PartialEq for ParamType<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.get() == other.0.get()
    }
}

impl<T: Default + Clone> Clone for ParamType<T> {
    fn clone(&self) -> Self {
        Self(UnsafeCell::new(
            unsafe { (self.0.get()).as_ref() }.unwrap().clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use super::*;

    #[derive(Default, Debug, PartialEq, Clone)]
    struct SomeParam;

    #[derive(Default)]
    struct SomeData {
        some_param: ParamType<SomeParam>,
    }

    impl SomeData {
        fn set_param(&self, param: &SomeParam) {
            *self.some_param.borrow_mut() = Some(param.clone())
        }
        pub fn get_param(&self) -> Ref<SomeParam> {
            self.some_param.borrow()
        }
    }

    #[test]
    fn set_param() {
        let param = SomeParam {};
        let data = SomeData::default();

        assert_eq!(data.some_param.is_none(), true);
        data.set_param(&param);
        assert_eq!(data.some_param.is_some(), true);
        assert_eq!(data.some_param.as_ref().unwrap(), &param);
    }

    #[test]
    fn get_param() {
        let param = SomeParam {};
        let data = SomeData::default();

        data.set_param(&param);
        assert_eq!(data.get_param().clone().as_ref().unwrap(), &param);
    }

    #[derive(Default, Debug, PartialEq, Clone)]
    struct SomeData1 {
        some_data2: ParamType<Box<SomeData2>>,
        value: u32,
    }

    impl SomeData1 {
        fn set_param(&self, param: &SomeData2) {
            *self.some_data2.borrow_mut() = Some(Box::new(param.clone()))
        }
        pub fn get_param(&self) -> Ref<Box<SomeData2>> {
            self.some_data2.borrow()
        }
    }

    #[derive(Default, Debug, PartialEq, Clone)]
    struct SomeData2 {
        some_data1: ParamType<Box<SomeData1>>,
        value: u32,
    }

    impl SomeData2 {
        fn set_param(&self, param: &SomeData1) {
            *self.some_data1.borrow_mut() = Some(Box::new(param.clone()))
        }
        pub fn get_param(&self) -> Ref<Box<SomeData1>> {
            self.some_data1.borrow()
        }
    }

    #[test]
    fn cross_ref() {
        let mut some_data1 = SomeData1::default();
        let mut some_data2 = SomeData2::default();

        assert!(some_data1.some_data2.is_none());
        assert!(some_data2.some_data1.is_none());

        some_data2.value = 2;
        some_data1.set_param(&some_data2);
        assert!(some_data1.some_data2.is_some());

        some_data1.value = 1;
        some_data2.set_param(&some_data1);
        assert!(some_data2.some_data1.is_some());

        assert_eq!(
            (*some_data1.get_param()).as_deref().unwrap().value,
            some_data2.value
        );
        assert_eq!(
            (*some_data2.get_param()).as_deref().unwrap().value,
            some_data1.value
        );
    }
}
