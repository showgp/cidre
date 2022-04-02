use std::ffi::{c_void, CStr};
use std::os::raw::c_char;
use std::ptr::NonNull;

use crate::cf::Retained;
use crate::define_obj_type;
use crate::dispatch::{Object, Function};

define_obj_type!(Queue(Object));
define_obj_type!(Global(Queue));
define_obj_type!(Serial(Queue));
define_obj_type!(Main(Serial));
define_obj_type!(Concurent(Queue));

define_obj_type!(Attr(Object));

#[repr(transparent)]
pub struct QOSClass(pub u32);

impl QOSClass {
    pub const USER_INTERACTIVE: Self = Self(0x21);
    pub const USER_INITIATED: Self = Self(0x19);
    pub const DEFAULT: Self = Self(0x15);
    pub const UTILITY: Self = Self(0x11);
    pub const BACKGROUND: Self = Self(0x09);
    pub const UNSPECIFIED: Self = Self(0x00);

    pub const QOS_MIN_RELATIVE_PRIORITY: i32 = -15;
}

#[repr(usize)]
pub enum AutoreleaseFrequency {
    Inherit = 0,
    WorkItem = 1,
    Never = 2,
}

/// ```
/// use cidre::dispatch;
///
/// let q = dispatch::Queue::main();
///
/// q.as_type_ref().show();
/// ```
impl Queue {
    #[inline]
    pub fn new<'a>() -> Retained<'a, Queue> {
        Self::with_label_and_attrs(None, None)
    }

    #[inline]
    pub fn with_label_and_attrs<'a>(
        label: Option<&CStr>,
        attr: Option<&Attr>,
    ) -> Retained<'a, Queue> {
        unsafe {
            let label = label.map(|f| NonNull::new_unchecked(f.as_ptr() as *mut _));
            dispatch_queue_create(label, attr)
        }
    }

    #[inline]
    pub fn main<'a>() -> &'a Queue {
        Main::default()
    }

    #[inline]
    pub fn async_f(&self, context: *mut c_void, work: Function) {
        unsafe { dispatch_async_f(self, context, work) }
    }

    #[inline]
    pub fn sync_f(&self, context: *mut c_void, work: Function) {
        unsafe { dispatch_sync_f(self, context, work) }
    }

    #[inline]
    pub fn async_and_wait_f(&self, context: *mut c_void, work: Function) {
        unsafe { dispatch_async_and_wait_f(self, context, work) }
    }

    #[inline]
    pub fn after_f(&self, when: super::Time, context: *mut c_void, work: Function) {
        unsafe { dispatch_after_f(when, self, context, work) }
    }

    #[inline]
    pub fn barrier_async_f(&self, context: *mut c_void, work: Function) {
        unsafe { dispatch_barrier_async_f(self, context, work) }
    }

    #[inline]
    pub fn barrier_sync_f(&self, context: *mut c_void, work: Function) {
        unsafe { dispatch_barrier_sync_f(self, context, work) }
    }

    #[inline]
    pub fn barrier_async_and_waitf(&self, context: *mut c_void, work: Function) {
        unsafe { dispatch_barrier_async_and_wait_f(self, context, work) }
    }

    #[inline]
    pub fn group_async_f(&self, group: &super::Group, context: *mut c_void, work: Function) {
        unsafe {
            dispatch_group_async_f(group, self, context, work)
        }
    }
}

impl Main {
    #[inline]
    fn default<'a>() -> &'a Main {
        unsafe { &_dispatch_main_q }
    }
}

impl Attr {
    #[inline]
    pub fn serial<'a>() -> Option<&'a Attr> {
        None
    }

    #[inline]
    pub fn concurrent<'a>() -> Option<&'a Attr> {
        unsafe { Some(&_dispatch_queue_attr_concurrent) }
    }

    #[inline]
    pub fn serial_inactive<'a>() -> Retained<'a, Attr> {
        Self::make_initially_inactive(Self::serial())
    }

    #[inline]
    pub fn concurrent_inactive<'a>() -> Retained<'a, Attr> {
        Self::make_initially_inactive(Self::concurrent())
    }

    #[inline]
    pub fn serial_with_autoreleasepool<'a>() -> Retained<'a, Attr> {
        Self::make_with_autorelease_frequencey(Self::serial(), AutoreleaseFrequency::WorkItem)
    }

    #[inline]
    pub fn concurrent_with_autoreleasepool<'a>() -> Retained<'a, Attr> {
        Self::make_with_autorelease_frequencey(Self::concurrent(), AutoreleaseFrequency::WorkItem)
    }

    #[inline]
    pub fn make_with_autorelease_frequencey<'a>(
        attr: Option<&Attr>,
        frequency: AutoreleaseFrequency,
    ) -> Retained<'a, Self> {
        unsafe { dispatch_queue_attr_make_with_autorelease_frequency(attr, frequency) }
    }

    #[inline]
    pub fn make_initially_inactive<'a>(attr: Option<&Attr>) -> Retained<'a, Attr> {
        unsafe { dispatch_queue_attr_make_initially_inactive(attr) }
    }

    #[inline]
    pub fn make_with_qos_class<'a>(
        attr: Option<&Attr>,
        qos_class: QOSClass,
        relative_priority: i32,
    ) -> Retained<'a, Attr> {
        unsafe { dispatch_queue_attr_make_with_qos_class(attr, qos_class, relative_priority) }
    }

    #[inline]
    pub fn initially_inactive<'a>(&self) -> Retained<'a, Attr> {
        unsafe { dispatch_queue_attr_make_initially_inactive(Some(self)) }
    }

    #[inline]
    pub fn with_autorelease_frequencey<'a>(
        &self,
        frequency: AutoreleaseFrequency,
    ) -> Retained<'a, Attr> {
        unsafe { dispatch_queue_attr_make_with_autorelease_frequency(Some(self), frequency) }
    }
}

extern "C" {
    static _dispatch_main_q: Main;
    static _dispatch_queue_attr_concurrent: Attr;

    fn dispatch_async_f(queue: &Queue, context: *mut c_void, work: Function);
    fn dispatch_sync_f(queue: &Queue, context: *mut c_void, work: Function);
    fn dispatch_queue_create<'a>(
        label: Option<NonNull<c_char>>,
        attr: Option<&Attr>,
    ) -> Retained<'a, Queue>;

    fn dispatch_async_and_wait_f(queue: &Queue, context: *mut c_void, work: Function);

    fn dispatch_queue_attr_make_initially_inactive<'a>(attr: Option<&Attr>) -> Retained<'a, Attr>;
    fn dispatch_queue_attr_make_with_qos_class<'a>(
        attr: Option<&Attr>,
        qos_class: QOSClass,
        relative_priority: i32,
    ) -> Retained<'a, Attr>;
    fn dispatch_queue_attr_make_with_autorelease_frequency<'a>(
        attr: Option<&Attr>,
        frequency: AutoreleaseFrequency,
    ) -> Retained<'a, Attr>;

    fn dispatch_after_f(
        when: crate::dispatch::Time,
        queue: &Queue,
        context: *mut c_void,
        work: Function,
    );
    fn dispatch_barrier_async_f(queue: &Queue, context: *mut c_void, work: Function);
    fn dispatch_barrier_sync_f(queue: &Queue, context: *mut c_void, work: Function);
    fn dispatch_barrier_async_and_wait_f(queue: &Queue, context: *mut c_void, work: Function);

    fn dispatch_group_async_f(group: &super::Group, queue: &Queue, context: *mut c_void, work: Function);
}

#[cfg(test)]
mod tests {

    use std::ffi::c_void;

    use crate::dispatch;

    extern "C" fn foo(_ctx: *mut c_void) {
        println!("nice");
    }

    #[test]
    fn test_attrs() {
        let _attr = dispatch::Attr::make_with_autorelease_frequencey(
            None,
            dispatch::AutoreleaseFrequency::Never,
        );
    }

    #[test]
    fn test_queue() {
        let q = dispatch::Queue::new();

        q.as_type_ref().show();

        q.sync_f(std::ptr::null_mut(), foo);

        q.async_and_wait_f(std::ptr::null_mut(), foo);
    }
}
