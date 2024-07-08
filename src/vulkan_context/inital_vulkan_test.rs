extern crate vulkano;
extern crate winit;

use std::{
    collections::HashMap, //Hashmap :)
    sync::Arc//Some heap allocated thing (Should read more)
};

//Use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize, 
    event::{
        ElementState,
        WindowEvent,
        KeyEvent
    }, 
    event_loop::{
        ActiveEventLoop, EventLoop
    }, window::{
        self, Window, WindowId
    }
};

use vulkano::{
    instance::{
        Instance,
        InstanceCreateInfo,
        InstanceExtensions
    },
    Version,
    VulkanLibrary
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct TriangleAppTest{//This is probably the application itself
    //instance: Option<Arc<Instance>>, Implement later
    parent_window_id: Option<WindowId>,
    windows: HashMap<WindowId, Window>,
}

impl ApplicationHandler for TriangleAppTest {

    fn window_event
        (
            &mut self,
            event_loop: &ActiveEventLoop,
            window_id: WindowId,
            event: WindowEvent,
        ){
        match event {
            WindowEvent::CloseRequested => {
                self.windows.clear();
                event_loop.exit();
            },
            WindowEvent::CursorEntered { device_id: _ } => {
                //You need to do something special on x11
                println!("cursor entered in the window {window_id:?}");
            },
            WindowEvent::KeyboardInput {
                event: KeyEvent { state: ElementState::Pressed, .. },
                ..
            } => {
                let parent_window = self.windows.get(&self.parent_window_id.unwrap()).unwrap();
                let child_window = window_id;//Spaw the window not use window id
                let child_id = child_window.id();
                println!("Child window created with id: {child_id:?}");
                self.windows.insert(child_id, child_window);
            },
            WindowEvent::RedrawRequested => {
                if let Some(window) = self.windows.get(&window_id) {

                    //They use
                    //fill::fill_window(window);
                    //I need to figure out what fill is
                }
            },
            _ => (),
        }
    }
}

/* 
impl TriangleAppTest{
    pub fn initialize() -> Self {
        let instance: Option<Arc<Instance>> = Self::create_instance();

        let mut windows: HashMap<WindowId, Window>;

        let main_event_loop = EventLoop::new().unwrap();

        let main_window: Window = main_event_loop.create_window(Window::default_attributes()).unwrap();
        
        Self{
            instance,
            
            windows,
        }
    }

    //This function creates a window titled Vulkan with the defined size
    //Not sure what events loop is yet (Should research)
    fn create_new_window() -> Window {
        let mut events_loop: EventLoop<()> = EventLoop::new().unwrap();
        /* */
        let _window = Some(events_loop.create_window(Window::default_attributes()))
            .with_title("Vulkan")//Window name
            .with_dimensions(LogicalSize::new(f64::from(WIDTH), f64::from(HEIGHT)))//Window size
            .build(&events_loop);
        //return events_loop;

        return _window;
    }

    fn create_instance() -> Option<Arc<Instance>>{

        let vk_library: Arc<VulkanLibrary> = VulkanLibrary::new().unwrap();

        //Creates a varible that contains the device extenstion support?
        let supported_extensions = 
            InstanceExtensions::empty();//I need to research how to acctually do this

        //Prints extenstion support for debug pourpases? (My spelling sucks)
        println!("Supported extenstions: {:?}", supported_extensions);
        /* 
        let app_info = ApplicationInfo{
            applictaion_name: Some("Triangle app".into()),//Should search more about Some and into()
            application_version: Some(Version{major: 1, minor: 0, patch: 0}),//version num? 1.00?
            engine_name: Some("No engine".into()),//Do not remember what this implies should google VKApplicationinfo pEngineName
            engine_version: Some(Version{major: 1, minor: 0, patch: 0}),
        };
        */

        //Remember to research this (Something about finding that extentions are needed)
        //let required_extensions = vulkano::required_extensions();

        //Create the instance using refrence to the varibles
        return Some( Instance::new(
            vk_library,
            InstanceCreateInfo::application_from_cargo_toml(),
        ).expect("Failed to create Vulkan instance"));
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
}*/

pub fn vulkan_instance(){
    let mut vulkan_app = TriangleAppTest::initialize();

    vulkan_app.main_loop();
}