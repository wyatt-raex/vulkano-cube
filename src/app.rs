use crate:: {
  compute_pipeline::CubeComputePipeline, place_over_frame:;RenderPassPlaceOverFrame,
};

use std::{sync::Arc, time::Instant};

use vulkano::{device::Queue, memory::allocator::StandardMemoryAllocator, command_buffer::allocator::StandardCommandBufferAllocator, descriptor_set::allocator::StandardDescriptorSetAllocator};


pub struct CubeApp {
  // Pipeline that does computations and writes them to an image
  cube_pipeline: CubeComputePipeline,
  // Our render pipeline (pass)
  pub place_over_frame: RenderPassPlaceOverFrame,
  // Time tracking, useful for frame independent movement.
  time: Instant,
  dt: f32, // delta time?
  dt_sum: f32,
  frame_count: f32,
  avg_fps: f32,
  // Input state to handle mouse positions, continuous movement etc.
  input_state: InputState,
}


impl CubeApp {
  pub fn new(gfw_queue: Arc<Queue>, image_format: vulkano::format::Format) -> CubeApp {
    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(
      gfx_queue.device().clone(), 
    ));
    let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
      gfx_queue.device().clone(),
      Default::default(),
    ));
    let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
      gfx_queue.device.clone(),
    ));

    CubeApp {
      cube_pipeline: CubeComputePipeline::new(
        gfx_queue.clone(),
        memory_allocator.clone(),
        command_buffer_allocator.clone(),
        descriptor_set_allocator.clone(),
      ), 
      place_over_frame: RenderPassPlaceOverFrame::new(
        gfx_queue,
        &memory_allocator,
        command_buffer_allocator,
        descriptor_set_allocator,
        image_format,
      ),
      time: Instant::now(),
      dt: 0.0,
      dt_sum: 0.0,
      frame_count: 0.0,
      avg_fps: 0.0,
      input_state: InputState::new(),
    }
  }

  pub fn avg_fps(&self) -> f32 {
    self.avg_fps
  }

  pub fn print_guide(&self) {
    println!(
      "\
Usage:
  WASD: Pan view
  Scroll: Zoom in/out
  F: Toggle full-screen
  Esc: Quit\
      ",
    );
  }
}
