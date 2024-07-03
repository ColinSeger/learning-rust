extern crate vulkano;
extern crate winit;

use winit::{EventsLoop, WindowBuilder, dpi::LogicalSize};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct TriangleAppTest{
    events_loop: EventsLoop,
}

impl TriangleAppTest{
    pub fn initialize() -> Self {
        let events_loop = Self::init_window();
        Self{
            events_loop,
        }
    }

    //This function creates a window titled Vulkan with the defined size
    //Not sure what events loop is yet (Should research)
    fn init_window() -> EventsLoop {
        let events_loop = EventsLoop::new();
        let _window = WindowBuilder::new()
            .with_title("Vulkan")
            .with_dimensions(LogicalSize::new(f64::from(WIDTH), f64::from(HEIGHT)))
            .build(&events_loop);
        events_loop
    }

    //If my understanding is correct this is a reapeating loop that check if any action is preformed
    fn main_loop(&mut self){
        loop {
            let mut done = false;
            self.events_loop.poll_events(|ev| {
                //If you try to close it sets done to true wich ends the loop
                if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = ev {
                    done = true
                }
            });
            if done {
                return;
            }
        }
    }
}

pub fn vulkan_instance(){
    let mut vulkan_app = TriangleAppTest::initialize();

    vulkan_app.main_loop();
}