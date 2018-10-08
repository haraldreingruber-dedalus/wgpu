use registry::{HUB, Items, Registry};
use {
    Stored,
    CommandBufferId, RenderPassId,
};

use hal;
use hal::command::RawCommandBuffer;


pub struct RenderPass<B: hal::Backend> {
    raw: B::CommandBuffer,
    cmb_id: Stored<CommandBufferId>,
}

impl<B: hal::Backend> RenderPass<B> {
    pub fn new(raw: B::CommandBuffer, cmb_id: CommandBufferId) -> Self {
        RenderPass {
            raw,
            cmb_id: Stored(cmb_id),
        }
    }
}

#[no_mangle]
pub extern "C" fn wgpu_render_pass_end_pass(
    pass_id: RenderPassId,
) -> CommandBufferId {
    let mut pass = HUB.render_passes
        .lock()
        .take(pass_id);
    pass.raw.end_render_pass();

    HUB.command_buffers
        .lock()
        .get_mut(pass.cmb_id.0)
        .raw = Some(pass.raw);
    pass.cmb_id.0
}