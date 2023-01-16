use std::{marker::PhantomData, mem::transmute, ops::Deref};

use crate::{
    arc, ns,
    objc::{msg_send, Class, Obj},
};

#[derive(Debug)]
#[repr(transparent)]
pub struct Set<T: Obj>(ns::Id, PhantomData<T>);

impl<T: Obj> Obj for Set<T> {}

#[derive(Debug)]
#[repr(transparent)]
pub struct SetMut<T: Obj>(ns::Set<T>);

impl<T: Obj> Obj for SetMut<T> {}

impl<T: Obj> Deref for Set<T> {
    type Target = ns::Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Obj> Deref for SetMut<T> {
    type Target = Set<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Obj> Set<T> {
    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { transmute(NS_SET.alloc_init()) }
    }

    #[inline]
    pub fn from_slice(objs: &[&T]) -> arc::R<Self> {
        unsafe {
            NS_SET
                .alloc()
                .call2(msg_send::init_with_objects_count, objs.as_ptr(), objs.len())
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        unsafe { self.call0(msg_send::count) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn iter(&self) -> ns::FEIterator<Self, T> {
        ns::FastEnumeration::iter(self)
    }
}

impl<T> ns::FastEnumeration<T> for Set<T> where T: Obj {}
impl<T> ns::FastEnumeration<T> for SetMut<T> where T: Obj {}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_SET: &'static Class<ns::Set<ns::Id>>;
}

#[cfg(test)]
mod tests {
    use crate::ns;
    #[test]
    fn basics() {
        let two = ns::Number::with_i32(10);
        let set: &[&ns::Number] = &[&two, &two, &two];
        let set = ns::Set::from_slice(set);
        assert_eq!(1, set.len());
        let sum = set.iter().map(|v| v.as_i32()).sum();
        assert_eq!(10, sum);
    }
}