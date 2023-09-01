use std::{sync::Arc};

use vulkano::{device::Queue};


pub struct CubeApp {}

impl CubeApp {
  pub fn new(gfw_queue: Arc<Queue>, image_format: vulkano::format::Format) -> CubeApp {
    CubeApp {}
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
