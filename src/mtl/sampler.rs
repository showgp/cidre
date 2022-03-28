use crate::{define_obj_type, objc::Id, define_mtl};

#[repr(usize)]
pub enum MinMagFilter {
    Nearest = 0,
    Linear = 1,
}

#[repr(usize)]
pub enum MipFilter {
    NotMipmapped = 0,
    Nearest = 1,
    Linear = 2,
}

#[repr(usize)]
pub enum AddressMode {
  ClampToEdge = 0,
  MirrorClampToEdge  = 1,
  Repeat = 2,
  MirrorRepeat = 3,
  ClampToZero = 4,
  ClampToBorderColor  = 5,
}

#[repr(usize)]
pub enum BorderColor {
  TransparentBlack = 0,  // {0,0,0,0}
  OpaqueBlack = 1,       // {0,0,0,1}
  OpaqueWhite = 2,       // {1,1,1,1}
}

define_obj_type!(Descriptor(Id));

define_obj_type!(State(Id));

impl State {
  define_mtl!(device, label);
}