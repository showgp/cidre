use crate::{define_mtl, define_obj_type, define_options, mtl, ns};

define_options!(BlitOption(usize));

impl BlitOption {
    pub const NONE: Self = Self(0);
    pub const DEPTH_FROM_DEPTH_STENCIL: Self = Self(1 << 0);
    pub const STENCIL_FROM_DEPTH_STENCIL: Self = Self(1 << 1);
    pub const ROW_LINEAR_PVRTC: Self = Self(1 << 2);
}

define_obj_type!(BlitCommandEncoder(mtl::CommandEncoder));

/// ```no_run
/// use cidre::{mtl};
///
/// let device = mtl::Device::default().unwrap();
///
/// let command_queue = device.command_queue().unwrap();
/// let command_buffer = command_queue.command_buffer().unwrap();
///
/// let fence = device.fence().unwrap();
///
/// let mut blit_encoder = command_buffer.blit_command_encoder().unwrap();
///
/// blit_encoder.update_fence(&fence);
/// blit_encoder.end_encoding();
///
/// let mut compute_encoder = command_buffer.compute_command_encoder().unwrap();
/// compute_encoder.wait_for_fence(&fence);
/// compute_encoder.end_encoding();
///
/// command_buffer.commit();
/// command_buffer.wait_until_completed();
///
/// ```
impl BlitCommandEncoder {
    define_mtl!(update_fence, wait_for_fence);

    #[inline]
    pub fn fill_buffer(&self, buffer: &mtl::Buffer, range: ns::Range, value: u8) {
        unsafe { wsel_fillBuffer(self, buffer, range, value) }
    }

    pub fn copy_texture(
        &self,
        src_texture: &mtl::Texture,
        src_slice: usize,
        src_level: usize,
        src_origin: mtl::Origin,
        src_size: mtl::Size,
        dest_texture: &mtl::Texture,
        dest_slice: usize,
        dest_level: usize,
        dest_origin: mtl::Origin,
    ) {
        unsafe {
            wsel_copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin(self, src_texture, src_slice, src_level, src_origin, src_size, dest_texture, dest_slice, dest_level, dest_origin)
        }
    }

    #[inline]
    pub fn copy_texture_to_texture(&self, src_texture: &mtl::Texture, dest_texture: &mtl::Texture) {
        unsafe { wsel_copyFromTexture_toTexture(self, src_texture, dest_texture) }
    }

    #[inline]
    pub fn optimize_contents_for_gpu_access(&self, texture: &mtl::Texture) {
        unsafe { wsel_optimizeContentsForGPUAccess(self, texture) }
    }

    #[inline]
    pub fn reset_commands_in_buffer_with_range(
        &self,
        buffer: &mtl::IndirectCommandBuffer,
        range: ns::Range,
    ) {
        unsafe { wsel_resetCommandsInBuffer_withRange(self, buffer, range) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn wsel_fillBuffer(id: &ns::Id, buffer: &mtl::Buffer, range: ns::Range, value: u8);

    fn wsel_copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin(
        id: &ns::Id,
        src_texture: &mtl::Texture,
        src_slice: usize,
        src_level: usize,
        src_origin: mtl::Origin,
        src_size: mtl::Size,
        dest_texture: &mtl::Texture,
        dest_slice: usize,
        dest_level: usize,
        dest_origin: mtl::Origin,
    );

    fn wsel_copyFromTexture_toTexture(
        id: &ns::Id,
        src_texture: &mtl::Texture,
        dest_texture: &mtl::Texture,
    );

    fn wsel_optimizeContentsForGPUAccess(id: &ns::Id, texture: &mtl::Texture);

    fn wsel_resetCommandsInBuffer_withRange(
        id: &ns::Id,
        buffer: &mtl::IndirectCommandBuffer,
        with_range: ns::Range,
    );
}

#[cfg(test)]
mod tests {
    use crate::mtl;

    #[test]
    fn basics() {
        let device = mtl::Device::default().unwrap();

        let command_queue = device.command_queue().unwrap();
        let command_buffer = command_queue.command_buffer().unwrap();

        let fence = device.fence().unwrap();

        let mut blit_encoder = command_buffer.blit_command_encoder().unwrap();

        blit_encoder.update_fence(&fence);
        blit_encoder.end_encoding();

        let mut compute_encoder = command_buffer.compute_command_encoder().unwrap();
        compute_encoder.wait_for_fence(&fence);
        compute_encoder.end_encoding();

        command_buffer.commit();
        command_buffer.wait_until_completed();
    }
}