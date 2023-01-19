use std::ffi::{c_char, c_long, c_void, CStr};
use std::mem::transmute;
use std::ptr::NonNull;

use crate::{
    arc, blocks, define_obj_type,
    dispatch::{self, Function},
};

define_obj_type!(Queue(dispatch::Object));
define_obj_type!(Global(Queue));
define_obj_type!(Serial(Queue));
define_obj_type!(Main(Serial));
define_obj_type!(Concurent(Queue));

define_obj_type!(Attr(dispatch::Object));

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Priority(pub c_long);

impl Priority {
    pub const HIGH: Self = Self(2);
    pub const DEFAULT: Self = Self(0);
    pub const LOW: Self = Self(-2);
    pub const BACKGROUND: Self = Self(-1 << 15);
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
    pub fn new() -> arc::R<Self> {
        Self::with_label_and_attrs(None, None)
    }

    #[inline]
    pub fn serial_with_autoreleasepool() -> arc::R<Self> {
        let attr = Attr::serial_with_autoreleasepool();
        Self::with_label_and_attrs(None, Some(&attr))
    }

    #[inline]
    pub fn with_label_and_attrs(label: Option<&CStr>, attr: Option<&Attr>) -> arc::R<Self> {
        unsafe {
            let label = label.map(|f| NonNull::new_unchecked(f.as_ptr() as *mut _));
            dispatch_queue_create(label, attr)
        }
    }

    #[inline]
    pub fn main() -> &'static Queue {
        Main::default()
    }

    /// ```
    /// use cidre::dispatch;
    ///
    /// let q = dispatch::Queue::global_with_qos(dispatch::QOSClass::BACKGROUND).unwrap();
    ///
    /// ```
    #[inline]
    pub fn global_with_qos(qos: QOSClass) -> Option<&'static Global> {
        unsafe { Self::global_with_flags(qos.0 as _, 0) }
    }

    /// ```
    /// use cidre::dispatch;
    ///
    /// let q = dispatch::Queue::global_with_priority(dispatch::QueuePriority::BACKGROUND).unwrap();
    ///
    /// ```
    #[inline]
    pub fn global_with_priority<'a>(priority: Priority) -> Option<&'a Global> {
        unsafe { Self::global_with_flags(priority.0 as _, 0) }
    }

    #[inline]
    pub fn global<'a>(identifier: isize) -> Option<&'a Global> {
        unsafe { Self::global_with_flags(identifier, 0) }
    }

    #[inline]
    pub unsafe fn global_with_flags<'a>(identifier: isize, flags: usize) -> Option<&'a Global> {
        dispatch_get_global_queue(identifier, flags)
    }

    #[inline]
    pub fn sync_b<F, B>(&self, block: &mut B)
    where
        B: dispatch::Block<F>,
    {
        unsafe {
            dispatch_sync(self, block.ptr());
        }
    }

    #[inline]
    pub fn async_b<F, B: dispatch::Block<F> + Sync>(&self, block: &'static mut B) {
        unsafe {
            dispatch_async(self, block.ptr());
        }
    }

    #[inline]
    pub fn sync_once<F: FnOnce() + 'static>(&self, f: F) {
        self.sync_b(blocks::once0(f).escape());
    }

    #[inline]
    pub fn sync_mut<F: FnMut() + 'static>(&self, f: F) {
        self.sync_b(&mut blocks::mut0(f));
    }

    #[inline]
    pub fn sync_fn(&self, block: extern "C" fn(*const c_void)) {
        let mut block = blocks::fn0(block);
        self.sync_b(&mut block);
    }

    #[inline]
    pub fn async_once<F: FnOnce() + Sync + 'static>(&self, block: F) {
        self.async_b(blocks::once0(block).escape());
    }

    #[inline]
    pub fn async_mut<F: FnMut() + Sync + 'static>(&self, block: F) {
        self.async_b(blocks::mut0(block).escape());
    }

    #[inline]
    pub fn async_fn(&self, block: extern "C" fn(*const c_void)) {
        let mut block = blocks::fn0(block);
        self.async_b(block.escape());
    }

    #[inline]
    pub fn async_f<T>(&self, context: *mut T, work: Function<T>) {
        unsafe { dispatch_async_f(self, context as _, transmute(work)) }
    }

    #[inline]
    pub fn sync_f<T>(&self, context: *mut T, work: Function<T>) {
        unsafe { dispatch_sync_f(self, context as _, transmute(work)) }
    }

    #[inline]
    pub fn async_and_wait_f<T>(&self, context: *mut T, work: Function<T>) {
        unsafe { dispatch_async_and_wait_f(self, context as _, transmute(work)) }
    }

    #[inline]
    pub fn after_f<T>(&self, when: super::Time, context: *mut T, work: Function<T>) {
        unsafe { dispatch_after_f(when, self, context as _, transmute(work)) }
    }

    #[inline]
    pub fn barrier_async_f<T>(&self, context: *mut T, work: Function<T>) {
        unsafe { dispatch_barrier_async_f(self, context as _, transmute(work)) }
    }

    #[inline]
    pub fn barrier_sync_f<T>(&self, context: *mut T, work: Function<T>) {
        unsafe { dispatch_barrier_sync_f(self, context as _, transmute(work)) }
    }

    #[inline]
    pub fn barrier_async_and_wait_f<T>(&self, context: *mut T, work: Function<T>) {
        unsafe { dispatch_barrier_async_and_wait_f(self, context as _, transmute(work)) }
    }

    #[inline]
    pub fn group_async_f<T>(&self, group: &super::Group, context: *mut T, work: Function<T>) {
        unsafe { dispatch_group_async_f(group, self, context as _, transmute(work)) }
    }

    pub const TARGET_QUEUE_DEFAULT: Option<&Self> = None;
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
    pub fn concurrent() -> Option<&'static Attr> {
        unsafe { Some(&_dispatch_queue_attr_concurrent) }
    }

    #[inline]
    pub fn serial_inactive() -> arc::R<Attr> {
        Self::make_initially_inactive(Self::serial())
    }

    #[inline]
    pub fn concurrent_inactive() -> arc::R<Attr> {
        Self::make_initially_inactive(Self::concurrent())
    }

    #[inline]
    pub fn serial_with_autoreleasepool() -> arc::R<Attr> {
        Self::make_with_autorelease_frequencey(Self::serial(), AutoreleaseFrequency::WorkItem)
    }

    #[inline]
    pub fn concurrent_with_autoreleasepool() -> arc::R<Attr> {
        Self::make_with_autorelease_frequencey(Self::concurrent(), AutoreleaseFrequency::WorkItem)
    }

    #[inline]
    pub fn make_with_autorelease_frequencey(
        attr: Option<&Attr>,
        frequency: AutoreleaseFrequency,
    ) -> arc::R<Self> {
        unsafe { dispatch_queue_attr_make_with_autorelease_frequency(attr, frequency) }
    }

    #[inline]
    pub fn make_initially_inactive(attr: Option<&Attr>) -> arc::R<Attr> {
        unsafe { dispatch_queue_attr_make_initially_inactive(attr) }
    }

    #[inline]
    pub fn make_with_qos_class(
        attr: Option<&Attr>,
        qos_class: QOSClass,
        relative_priority: i32,
    ) -> arc::R<Attr> {
        unsafe { dispatch_queue_attr_make_with_qos_class(attr, qos_class, relative_priority) }
    }

    #[inline]
    pub fn initially_inactive(&self) -> arc::R<Attr> {
        unsafe { dispatch_queue_attr_make_initially_inactive(Some(self)) }
    }

    #[inline]
    pub fn with_autorelease_frequencey(&self, frequency: AutoreleaseFrequency) -> arc::R<Attr> {
        unsafe { dispatch_queue_attr_make_with_autorelease_frequency(Some(self), frequency) }
    }
}

#[link(name = "System", kind = "dylib")]
extern "C" {
    static _dispatch_main_q: Main;
    static _dispatch_queue_attr_concurrent: Attr;

    fn dispatch_sync(queue: &Queue, block: *mut c_void);
    fn dispatch_async(queue: &Queue, block: *mut c_void);

    fn dispatch_async_f(queue: &Queue, context: *mut c_void, work: Function<c_void>);
    fn dispatch_sync_f(queue: &Queue, context: *mut c_void, work: Function<c_void>);
    fn dispatch_queue_create(label: Option<NonNull<c_char>>, attr: Option<&Attr>) -> arc::R<Queue>;

    fn dispatch_async_and_wait_f(queue: &Queue, context: *mut c_void, work: Function<c_void>);

    fn dispatch_queue_attr_make_initially_inactive(attr: Option<&Attr>) -> arc::R<Attr>;
    fn dispatch_queue_attr_make_with_qos_class(
        attr: Option<&Attr>,
        qos_class: QOSClass,
        relative_priority: i32,
    ) -> arc::R<Attr>;
    fn dispatch_queue_attr_make_with_autorelease_frequency(
        attr: Option<&Attr>,
        frequency: AutoreleaseFrequency,
    ) -> arc::R<Attr>;

    fn dispatch_after_f(
        when: crate::dispatch::Time,
        queue: &Queue,
        context: *mut c_void,
        work: Function<c_void>,
    );
    fn dispatch_barrier_async_f(queue: &Queue, context: *mut c_void, work: Function<c_void>);
    fn dispatch_barrier_sync_f(queue: &Queue, context: *mut c_void, work: Function<c_void>);
    fn dispatch_barrier_async_and_wait_f(
        queue: &Queue,
        context: *mut c_void,
        work: Function<c_void>,
    );

    fn dispatch_group_async_f(
        group: &super::Group,
        queue: &Queue,
        context: *mut c_void,
        work: Function<c_void>,
    );
    fn dispatch_get_global_queue<'a>(identifier: isize, flags: usize) -> Option<&'a Global>;
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

    #[derive(Debug)]
    struct Foo {}

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("drop!")
        }
    }

    #[test]
    fn test_sync_block() {
        let q = dispatch::Queue::new();

        let foo = Foo {};
        q.as_type_ref().show();
        let b = move || {
            println!("nice! {:?}", foo);
        };
        q.async_once(b);

        let foo2 = Foo {};
        q.sync_mut(move || {
            println!("nice {:?}", foo2);
        });
    }

    #[test]
    fn test_global_queue() {
        let q = dispatch::Queue::global_with_qos(dispatch::QOSClass::BACKGROUND).unwrap();

        q.as_type_ref().show();
        q.sync_f(std::ptr::null_mut(), foo);
        q.async_and_wait_f(std::ptr::null_mut(), foo);

        let q = dispatch::Queue::global_with_priority(dispatch::QueuePriority::HIGH).unwrap();

        q.as_type_ref().show();
        q.sync_f(std::ptr::null_mut(), foo);
        q.async_and_wait_f(std::ptr::null_mut(), foo);
    }
}