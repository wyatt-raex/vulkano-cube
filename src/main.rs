// This program will end up rendering a cube to the window and allowing the user to control an
// orbiting camera around it. The project structure/code will be modeled after the `interactive
// fractal` example code for vulkano:
// 
// https://github.com/vulkano-rs/vulkano/tree/0.33.X/examples/src/bin/interactive_fractal
//
// This code will also be heavily commented to be used as future reference as I continue to learn 
// Vulkano.

use crate::app::CubeApp;

use vulkano::{image::ImageUsage, swapchain::PresentMode};

use vulkano_util::{
  context::{VulkanoConfig, VulkanoContext},
  renderer::{DEFAULT_IMAGE_FORMAT},
  window::{VulkanoWindows, WindowDescriptor},
};

use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
};

mod app;
mod compute_pipeline;


fn main() {
  // Create the event loop
  let mut event_loop = EventLoop::new();

  // `context` is a utility struct that creates & allows us to access our Vulkano device, instance,
  // and queues.
  //
  // `windows` is a utility struct that creates and organizes windows and their corresponding
  // renderers.
  let context = VulkanoContext::new(VulkanoConfig::default());
  let mut windows = VulkanoWindows::default();

  // Create winit window via `VulkanoWindows` crate
  let _id = windows.create_window(
    &event_loop,
    &context,
    &WindowDescriptor {
      resizable: false, // Force my i3 window manager to float this window
      title: "Vulkano Cube".to_string(),
      present_mode: PresentMode::Fifo,
      ..Default::default()
    },
    |_| {},
  );

  // Add our render target image onto which we'll be rendering our cube.
  //
  // An `image` is the "frame" we are rendering to the screen.
  // An `image view` is a section of said image we can edit and manipulate. Think as if your taking
  // a screenshot of a game. You are taking a `image view` of a portion of the game screen (the
  // "image").
  //
  // `get_primary_renderer_mut()` returns a mutable reference to the primary window renderer
  // struct. Said struct holds the winit window surface & functionality for organizing your
  // renderer between frames.
  let render_target_id = 0;
  let primary_window_renderer = windows.get_primary_renderer_mut().unwrap();

  // Make sure the image usage is correct (based on your pipeline).
  primary_window_renderer.add_additional_image_view(
    render_target_id,
    DEFAULT_IMAGE_FORMAT,
    ImageUsage::SAMPLED | ImageUsage::STORAGE | ImageUsage::TRANSFER_DST,
  );

  // Get the graphics queue to pass to our app. This will hold the logic to render our cube.
  let gfx_queue = context.graphics_queue();

  // Intend to eventually render on our swapchain, thus we use that format when creating the app
  // here.
  let mut app = CubeApp::new(
    gfx_queue.clone(),
    primary_window_renderer.swapchain_format(),
  );
  app.print_guide();

  // Basic loop for our runtime:
  // 1. Handle events
  // 2. Update state based on events
  // 3. Compute & Render
  // 4. Reset input state
  // 5. Update time & title
  loop {
    if !handle_events(&mut event_loop, primary_window_renderer, &mut app) { break; }

    match primary_window_renderer.window_size() {
      [w, h] => {
        // Skip this frame when minimized
        if w == 0.0 || h == 0.0 {
          continue;
        }
      }
    }

    app.update_state_after_inputs(primary_window_renderer);
    computer_then_render(primary_window_renderer, &mut app, render_target_id);
    app.reset_input_state();
    primary_window_renderer.window().set_title(&format!(
      "Vulkano Cube fps: {:.2}",
      app.avg_fps(),
    ));
  }
}
