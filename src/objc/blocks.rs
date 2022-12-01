// TODO:
// 1. Pure fn block is global _NSConcreteGlobalBlock
// 2. Pure fn block with fields - _NSConcreteMallocBlock
// 3. NoEscaping block - _NSConcreteStackBlock

// https://opensource.apple.com/source/libclosure/libclosure-79/BlockImplementation.txt.auto.html
// https://github.com/apple-oss-distributions/libclosure/blob/main/BlockImplementation.txt
// https://developer.apple.com/documentation/swift/calling-objective-c-apis-asynchronously

use std::{ffi::c_void, mem, ops};

use crate::{define_options, objc::Class};

#[repr(transparent)]
pub struct Block<F>(c_void, std::marker::PhantomData<F>);

impl<F> Block<F> {
    #[inline]
    pub unsafe fn as_ptr(&mut self) -> *mut c_void {
        self as *mut Self as _
    }
}

pub fn fn0<R>(f: extern "C" fn(*const c_void) -> R) -> bl<extern "C" fn(*const c_void) -> R> {
    bl::with(f)
}

pub fn fn1<A, R>(
    f: extern "C" fn(*const c_void, a: A) -> R,
) -> bl<extern "C" fn(*const c_void, a: A) -> R> {
    bl::with(f)
}

pub fn fn2<A, B, R>(
    f: extern "C" fn(*const c_void, a: A, b: B) -> R,
) -> bl<extern "C" fn(*const c_void, a: A, b: B) -> R> {
    bl::with(f)
}

pub fn fn3<A, B, C, R>(
    f: extern "C" fn(*const c_void, a: A, b: B, c: C) -> R,
) -> bl<extern "C" fn(*const c_void, a: A, b: B, c: C) -> R> {
    bl::with(f)
}

pub fn fn4<A, B, C, D, R>(
    f: extern "C" fn(*const c_void, a: A, b: B, c: C, d: D) -> R,
) -> bl<extern "C" fn(*const c_void, a: A, b: B, c: C, d: D) -> R> {
    bl::with(f)
}

pub fn fn5<A, B, C, D, E, R>(
    f: extern "C" fn(*const c_void, a: A, b: B, c: C, d: D, e: E) -> R,
) -> bl<extern "C" fn(*const c_void, a: A, b: B, c: C, d: D, e: E) -> R> {
    bl::with(f)
}

pub fn once0<R, F: 'static>(f: F) -> BlOnce<F>
where
    F: FnOnce() -> R,
{
    OnceLayout2::new(OnceLayout2::<F>::invoke0 as _, f)
}

pub fn once1<A, R, F: 'static>(f: F) -> BlOnce<F>
where
    F: FnOnce(A) -> R,
{
    OnceLayout2::new(OnceLayout2::<F>::invoke1 as _, f)
}

pub fn once2<A, B, R, F: 'static>(f: F) -> BlOnce<F>
where
    F: FnOnce(A, B) -> R,
{
    OnceLayout2::new(OnceLayout2::<F>::invoke2 as _, f)
}

pub fn once3<A, B, C, R, F: 'static>(f: F) -> BlOnce<F>
where
    F: FnOnce(A, B, C) -> R,
{
    OnceLayout2::new(OnceLayout2::<F>::invoke3 as _, f)
}

pub fn once4<A, B, C, D, R, F: 'static>(f: F) -> BlOnce<F>
where
    F: FnOnce(A, B, C, D) -> R,
{
    OnceLayout2::new(OnceLayout2::<F>::invoke4 as _, f)
}

pub fn once5<A, B, C, D, E, R, F: 'static>(f: F) -> BlOnce<F>
where
    F: FnOnce(A, B, C, D, E) -> R,
{
    OnceLayout2::new(OnceLayout2::<F>::invoke5 as _, f)
}

pub fn mut0<R, F: 'static>(f: F) -> BlMut<F>
where
    F: FnMut() -> R,
{
    MutLayout2::new(MutLayout2::<F>::invoke0 as _, f)
}

pub fn mut1<A, R, F: 'static>(f: F) -> BlMut<F>
where
    F: FnMut(A) -> R,
{
    MutLayout2::new(MutLayout2::<F>::invoke1 as _, f)
}

pub fn mut2<A, B, R, F: 'static>(f: F) -> BlMut<F>
where
    F: FnMut(A, B) -> R,
{
    MutLayout2::new(MutLayout2::<F>::invoke2 as _, f)
}

pub fn mut3<A, B, C, R, F: 'static>(f: F) -> BlMut<F>
where
    F: FnMut(A, B, C) -> R,
{
    MutLayout2::new(MutLayout2::<F>::invoke3 as _, f)
}

pub fn mut4<A, B, C, D, R, F: 'static>(f: F) -> BlMut<F>
where
    F: FnMut(A, B, C, D) -> R,
{
    MutLayout2::new(MutLayout2::<F>::invoke4 as _, f)
}

pub fn mut5<A, B, C, D, E, R, F: 'static>(f: F) -> BlMut<F>
where
    F: FnMut(A, B, C, D, E) -> R,
{
    MutLayout2::new(MutLayout2::<F>::invoke5 as _, f)
}

define_options!(Flags(i32));

impl Flags {
    pub const NONE: Self = Self(0);

    // runtime
    pub const DEALLOCATING: Self = Self(1);

    // runtime
    pub const REFCOUNT_MASK: Self = Self(0xfffei32);

    // compiler
    // Set to true on blocks that have captures (and thus are not true
    // global blocks) but are known not to escape for various other
    // reasons. For backward compatibility with old runtimes, whenever
    // IS_NOESCAPE is set, IS_GLOBAL is set too. Copying a
    // non-escaping block returns the original block and releasing such a
    // block is a no-op, which is exactly how global blocks are handled.
    pub const IS_NOESCAPE: Self = Self(1 << 23);

    // runtime
    pub const NEEDS_FREE: Self = Self(1 << 24);
    // compiler
    pub const HAS_COPY_DISPOSE: Self = Self(1 << 25);
    pub const HAS_CTOR: Self = Self(1 << 26);
    pub const IS_GC: Self = Self(1 << 27);
    pub const IS_GLOBAL: Self = Self(1 << 28);
    pub const USE_STRET: Self = Self(1 << 29);
    pub const HAS_SIGNATURE: Self = Self(1 << 30);
    pub const HAS_EXTENDED_LAYOUT: Self = Self(1 << 31);
}

#[repr(C)]
pub struct Descriptor1 {
    reserved: usize,
    size: usize,
}

#[repr(C)]
pub struct Descriptor2<T: Sized> {
    descriptor1: Descriptor1,
    copy: extern "C" fn(dest: &mut T, src: &mut T),
    dispose: extern "C" fn(liteal: &mut T),
}

// for completion handlers
#[repr(C)]
struct OnceLayout2<F: Sized + 'static> {
    isa: &'static Class,
    flags: Flags,
    reserved: i32,
    invoke: *const c_void,
    descriptor: &'static Descriptor2<Self>,
    closure: Option<F>,
}

// #[repr(C)]
// struct OnceLayout1<F: Sized + 'static> {
//     isa: &'static Class,
//     flags: Flags,
//     reserved: i32,
//     invoke: *const c_void,
//     descriptor: &'static Descriptor1,
//     closure: mem::ManuallyDrop<F>,
// }

#[repr(C)]
struct MutLayout2<F: Sized + 'static> {
    isa: &'static Class,
    flags: Flags,
    reserved: i32,
    invoke: *const c_void,
    descriptor: &'static Descriptor2<Self>,
    closure: mem::ManuallyDrop<F>,
}

unsafe impl<F:Sized + 'static> Sync for OnceLayout2<F> where F: Sync {}
unsafe impl<F:Sized + 'static> Sync for MutLayout2<F> where F: Sync {}

/// block with static fn
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct bl<F: Sized> {
    isa: &'static Class,
    flags: Flags,
    reserved: i32,
    invoke: F,
    descriptor: &'static Descriptor1,
}

impl<F: Sized> bl<F> {
    #[inline]
    pub fn escape<'a>(&mut self) -> &'a mut Block<F> {
        unsafe { mem::transmute(self) }
    }
}

#[repr(transparent)]
pub struct BlOnce<F: 'static>(&'static mut OnceLayout2<F>);

impl<F> BlOnce<F> {
    #[inline]
    pub fn escape<'a>(self) -> &'a mut Block<F> {
        let res = self.0 as *mut OnceLayout2<F>;
        std::mem::forget(self);
        unsafe { mem::transmute(res) }
    }
}

#[repr(transparent)]
pub struct BlMut<F: 'static>(&'static mut MutLayout2<F>);

impl<F> BlMut<F> {
    #[inline]
    pub fn escape<'a>(&mut self) -> &'a mut Block<F> {
        let res = self.0 as *mut MutLayout2<F>;
        unsafe { mem::transmute(res) }
    }
}

impl<F> Drop for BlOnce<F> {
    #[inline]
    fn drop(&mut self) {
        self.0.closure.take();
        unsafe {
            _Block_release(self.0 as *mut _ as *const _);
        };
    }
}

impl<F> Drop for BlMut<F> {
    #[inline]
    fn drop(&mut self) {
        unsafe { _Block_release(self.0 as *mut _ as *const _) };
    }
}

impl<F> Clone for BlMut<F> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            let ptr = _Block_copy(mem::transmute(self));
            Self(mem::transmute(ptr))
        }
    }
}

impl<F> ops::Deref for BlOnce<F> {
    type Target = Block<F>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let res = self.0 as *const OnceLayout2<F>;
        unsafe { mem::transmute(res) }
    }
}

impl<F> ops::Deref for BlMut<F> {
    type Target = Block<F>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let res = self.0 as *const MutLayout2<F>;
        unsafe { mem::transmute(res) }
    }
}

impl<F> ops::DerefMut for BlMut<F> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let res = self.0 as *mut MutLayout2<F>;
        unsafe { mem::transmute(res) }
    }
}

impl<F> ops::Deref for bl<F> {
    type Target = Block<F>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(self) }
    }
}

impl<F> ops::DerefMut for bl<F> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { mem::transmute(self) }
    }
}

impl<F: Sized> OnceLayout2<F> {
    const DESCRIPTOR_2: Descriptor2<Self> = Descriptor2 {
        descriptor1: Descriptor1 {
            reserved: 0,
            size: std::mem::size_of::<Self>(),
        },
        copy: Self::copy,
        dispose: Self::dispose,
    };

    extern "C" fn copy(_dest: &mut Self, _src: &mut Self) {
        panic!("copy should not be called");
    }

    extern "C" fn dispose(block: &mut Self) {
        block.closure.take();
    }

    extern "C" fn invoke0<R>(&mut self) -> R
    where
        F: FnOnce() -> R,
    {
        if let Some(closure) = self.closure.take() {
            (closure)()
        } else {
            panic!()
        }
    }

    extern "C" fn invoke1<A, R>(&mut self, a: A) -> R
    where
        F: FnOnce(A) -> R,
    {
        if let Some(closure) = self.closure.take() {
            (closure)(a)
        } else {
            panic!()
        }
    }

    extern "C" fn invoke2<A, B, R>(&mut self, a: A, b: B) -> R
    where
        F: FnOnce(A, B) -> R,
    {
        if let Some(closure) = self.closure.take() {
            (closure)(a, b)
        } else {
            panic!()
        }
    }

    extern "C" fn invoke3<A, B, C, R>(&mut self, a: A, b: B, c: C) -> R
    where
        F: FnOnce(A, B, C) -> R,
    {
        if let Some(closure) = self.closure.take() {
            (closure)(a, b, c)
        } else {
            panic!()
        }
    }

    extern "C" fn invoke4<A, B, C, D, R>(&mut self, a: A, b: B, c: C, d: D) -> R
    where
        F: FnOnce(A, B, C, D) -> R,
    {
        if let Some(closure) = self.closure.take() {
            (closure)(a, b, c, d)
        } else {
            panic!()
        }
    }

    extern "C" fn invoke5<A, B, C, D, E, R>(&mut self, a: A, b: B, c: C, d: D, e: E) -> R
    where
        F: FnOnce(A, B, C, D, E) -> R,
    {
        if let Some(closure) = self.closure.take() {
            (closure)(a, b, c, d, e)
        } else {
            panic!()
        }
    }

    // const NEW_FLAGS: Flags = Flags(Flags::NEEDS_FREE.0 | Flags(2).0); // logical retain count 1
    const NEW_FLAGS: Flags = Flags(Flags::HAS_COPY_DISPOSE.0 | Flags::NEEDS_FREE.0 | Flags(2).0); // logical retain count 1

    fn new(invoke: *const c_void, f: F) -> BlOnce<F> {
        let block = Box::new(Self {
            isa: unsafe { &_NSConcreteMallocBlock },
            flags: Self::NEW_FLAGS,
            reserved: 0,
            invoke,
            descriptor: &Self::DESCRIPTOR_2,
            closure: Some(f),
        });
        BlOnce(Box::leak(block))
    }
}

impl<F: Sized> MutLayout2<F> {
    const DESCRIPTOR_2: Descriptor2<Self> = Descriptor2 {
        descriptor1: Descriptor1 {
            reserved: 0,
            size: std::mem::size_of::<Self>(),
        },
        copy: Self::copy,
        dispose: Self::dispose,
    };

    extern "C" fn copy(_dest: &mut Self, _src: &mut Self) {
        panic!("copy should not be called");
    }

    extern "C" fn dispose(block: &mut Self) {
        unsafe {
            mem::ManuallyDrop::drop(&mut block.closure);
        }
    }

    extern "C" fn invoke0<R>(&mut self) -> R
    where
        F: FnMut() -> R,
    {
        (self.closure)()
    }

    extern "C" fn invoke1<A, R>(&mut self, a: A) -> R
    where
        F: FnMut(A) -> R,
    {
        (self.closure)(a)
    }

    extern "C" fn invoke2<A, B, R>(&mut self, a: A, b: B) -> R
    where
        F: FnMut(A, B) -> R,
    {
        (self.closure)(a, b)
    }

    extern "C" fn invoke3<A, B, C, R>(&mut self, a: A, b: B, c: C) -> R
    where
        F: FnMut(A, B, C) -> R,
    {
        (self.closure)(a, b, c)
    }

    extern "C" fn invoke4<A, B, C, D, R>(&mut self, a: A, b: B, c: C, d: D) -> R
    where
        F: FnMut(A, B, C, D) -> R,
    {
        (self.closure)(a, b, c, d)
    }

    extern "C" fn invoke5<A, B, C, D, E, R>(&mut self, a: A, b: B, c: C, d: D, e: E) -> R
    where
        F: FnMut(A, B, C, D, E) -> R,
    {
        (self.closure)(a, b, c, d, e)
    }

    const NEW_FLAGS: Flags = Flags(Flags::HAS_COPY_DISPOSE.0 | Flags::NEEDS_FREE.0 | Flags(2).0); // logical retain count 1

    fn new(invoke: *const c_void, f: F) -> BlMut<F> {
        let block = Box::new(Self {
            isa: unsafe { &_NSConcreteMallocBlock },
            flags: Self::NEW_FLAGS,
            reserved: 0,
            invoke,
            descriptor: &Self::DESCRIPTOR_2,
            closure: mem::ManuallyDrop::new(f),
        });
        BlMut(Box::leak(block))
    }
}

impl<F> bl<F> {
    const DESCRIPTOR: Descriptor1 = Descriptor1 {
        reserved: 0,
        size: std::mem::size_of::<Self>(),
    };

    pub fn with(f: F) -> bl<F> {
        bl {
            isa: unsafe { &_NSConcreteStackBlock },
            flags: Flags::NONE,
            reserved: 0,
            invoke: f,
            descriptor: &Self::DESCRIPTOR,
        }
    }
}

#[cfg_attr(
    any(target_os = "macos", target_os = "ios"),
    link(name = "System", kind = "dylib")
)]
#[cfg_attr(
    not(any(target_os = "macos", target_os = "ios")),
    link(name = "BlocksRuntime", kind = "dylib")
)]
extern "C" {
    static _NSConcreteGlobalBlock: Class;
    static _NSConcreteStackBlock: Class;
    static _NSConcreteMallocBlock: Class;

    fn _Block_copy(block: *const c_void) -> *const c_void;
    fn _Block_release(block: *const c_void);
}

#[cfg(test)]
mod tests {
    use crate::objc::blocks;

    #[derive(Debug)]
    struct Foo;

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("dropped foo");
        }
    }

    #[test]
    fn test_simple_block() {
        let foo = Foo;
        let mut b = blocks::mut0(move || println!("nice {foo:?}"));

        let q = crate::dispatch::Queue::new();
        q.async_b(b.escape());
        q.async_b(b.escape());
        q.async_mut(|| println!("nice"));
        q.sync_mut(|| println!("fuck"));

        println!("finished");
    }
}

use parking_lot::Mutex;
use std::sync::Arc;

use crate::cf::{self, runtime::Retain, Retained};

struct Shared<T> {
    ready: Option<T>,
    pending: Option<std::task::Waker>,
}

impl<T> Shared<T> {
    fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            ready: None,
            pending: None,
        }))
    }

    pub fn ready(&mut self, result: T) {
        self.ready = Some(result);

        if let Some(waker) = self.pending.take() {
            waker.wake();
        }
    }
}

pub struct Comp<R>(Arc<Mutex<Shared<R>>>);

impl<T> std::future::Future for Comp<T> {
    type Output = T;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut lock = self.0.lock();

        if let Some(r) = lock.ready.take() {
            std::task::Poll::Ready(r)
        } else {
            lock.pending = Some(cx.waker().clone());
            std::task::Poll::Pending
        }
    }
}

fn async_comp0() -> (Comp<()>, BlOnce<impl FnOnce()>) {
    let shared = Shared::new();
    let comp = Comp(shared.clone());
    let block = once0(move || {
        shared.lock().ready(());
    });

    (comp, block)
}

pub fn ok() -> (
    Comp<Result<(), Retained<cf::Error>>>,
    BlOnce<impl FnOnce(Option<&'static cf::Error>)>,
) {
    let shared = Shared::new();
    (
        Comp(shared.clone()),
        once1(move |error: Option<&'static cf::Error>| {
            shared.lock().ready(match error {
                Some(err) => Err(err.retained()),
                None => Ok(()),
            });
        }),
    )
}

pub fn result<T: Retain>() -> (
    Comp<Result<Retained<T>, Retained<cf::Error>>>,
    BlOnce<impl FnOnce(Option<&'static T>, Option<&'static cf::Error>)>,
) {
    let shared = Shared::new();
    (
        Comp(shared.clone()),
        once2(
            move |value: Option<&'static T>, error: Option<&'static cf::Error>| {
                let res = match error {
                    Some(err) => Err(err.retained()),
                    None => Ok(unsafe { value.unwrap_unchecked().retained() }),
                };

                shared.lock().ready(res);
            },
        ),
    )
}
