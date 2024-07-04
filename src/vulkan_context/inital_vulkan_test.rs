extern crate vulkano;
extern crate winit;

use std::sync::Arc;//Some heap allocated thing (Should read more)

use winit::{
    EventsLoop, 
    WindowBuilder, 
    dpi::LogicalSize
};

use vulkano::instance::{
    Instance,
    InstanceExtensions,
    ApplicationInfo,
    Version,
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct TriangleAppTest{//This is probably the application itself
    instance: Option<Arc<Instance>>,
    events_loop: EventsLoop,
}

impl TriangleAppTest{
    pub fn initialize() -> Self {
        let instance = Self::create_instance();
        let events_loop = Self::init_window();
        Self{
            instance,
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

    fn create_instance() -> Arc<Instance>{
        //Creates a varible that contains the device extenstion support?
        let supported_extensions = 
            InstanceExtensions::supported_by_core()
            .expect("Failed to retrive supported extenstions");

        //Prints extenstion support for debug pourpases? (My spelling sucks)
        printls!("Supported extenstions: {:?}", supported_extensions);

        let app_info = ApplicationInfo{
            applictaion_name: Some("Triangle app".into()),//Should search more about Some and into()
            application_version: Some(Version{major: 1, minor: 0, patch: 0}),//version num? 1.00?
            engine_name: Some("No engine".into()),//Do not remember what this implies should google VKApplicationinfo pEngineName
            engine_version: Some(Version{major: 1, minor: 0, patch: 0}),
        };

        //Remember to research this (Something about finding that extentions are needed)
        let required_extensions = vulkano::required_extensions();

        //Create the instance using refrence to the varibles
        Instance::new(
            Some(&app_info),
             &required_extensions,
            None
        ).expect("Failed to create Vulkan instance");
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