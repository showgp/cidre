use crate::{
    define_obj_type, ns,
    objc::{msg_send, Obj},
};

define_obj_type!(FunctionDescriptor(ns::Id));

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
pub enum FunctionOptions {
    None = 0,
    CompileToBinary = 1 << 0,
}

impl Default for FunctionOptions {
    fn default() -> Self {
        FunctionOptions::None
    }
}

impl FunctionDescriptor {
    #[inline]
    pub fn name(&self) -> Option<&ns::String> {
        unsafe { self.call0(msg_send::name) }
    }

    #[inline]
    pub fn set_name(&mut self, name: Option<&ns::String>) {
        unsafe { self.call1(msg_send::set_name, name) }
    }

    /// ```no_run
    /// use cidre::{ns, mtl};
    ///
    /// let fd = mtl::FunctionDescriptor::default();
    ///
    /// assert!(fd.name().is_none());
    ///
    /// let name = ns::String::with_str("hello");
    ///
    /// fd.set_name(Some(&name));
    ///
    /// let actual_name = fd.name().unwrap();
    ///
    /// assert!(name.is_equal(&actual_name));
    /// ```
    #[inline]
    pub fn default<'a>() -> &'a mut FunctionDescriptor {
        unsafe { MTLFunctionDescriptor_functionDescriptor() }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn MTLFunctionDescriptor_functionDescriptor<'a>() -> &'a mut FunctionDescriptor;
}

#[cfg(test)]
mod tests {

    use crate::{mtl, ns};

    #[test]
    fn basics() {
        let fd = mtl::FunctionDescriptor::default();

        assert!(fd.name().is_none());

        let name = ns::String::with_str("hello");

        fd.set_name(Some(&name));

        let actual_name = fd.name().unwrap();

        assert!(name.is_equal(&actual_name));
    }
}