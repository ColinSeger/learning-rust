extern crate vulkano;
extern crate winit;

use std::{
    collections::HashMap, error::Error, sync::Arc//Some heap allocated thing (Should read more)
};

use rwh_06::{DisplayHandle, HasDisplayHandle};

use softbuffer::{Context, Surface};

use winit::{
    application::ApplicationHandler, 
    dpi::{
        LogicalPosition, 
        LogicalSize, 
        Position
    }, 
    event::{
        ElementState,
        KeyEvent, 
        WindowEvent
    }, 
    event_loop::{
        self, ActiveEventLoop, EventLoop
    }, 
    raw_window_handle::HasWindowHandle, 
    window::{
        self, 
        Window, 
        WindowId
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
    //parent_window_id: Option<WindowId>,
    windows: HashMap<WindowId, Window>,
    context: Option<Context<DisplayHandle<'static>>>,
    //window: Window

}

impl TriangleAppTest {

    fn initialize(event_loop: &EventLoop<()>) -> Self {
        // SAFETY: we drop the context right before the event loop is stopped, thus making it safe.
        let context = Some(
            Context::new(unsafe {
                std::mem::transmute::<DisplayHandle<'_>, DisplayHandle<'static>>(
                    event_loop.display_handle().unwrap(),
                )
            })
            .unwrap(),
        );

        Self{
            windows: Default::default(),
            context: context,
        }
    }

    fn create_window
    (
        &mut self, 
        event_loop: &ActiveEventLoop, 
        _tab_id: Option<String>
    )-> Result<WindowId, Box<dyn Error>>{

        let window_attributes = Window::default_attributes()
        .with_title("window title")
        .with_transparent(true)
        //.with_window_icon(Some(()))
        ;
        
        let window = event_loop.create_window(window_attributes)?;

        //let window_state = WindowState::new(self, window)?;
        let window_id = window.id();//Should use a window state
        //info!("Created new window with id={window_id:?}");
        self.windows.insert(window_id, window);//Should create a window state
        Ok(window_id)
    }
}

/* 
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
        fn spawn_child_window(parent: &Window, event_loop: &ActiveEventLoop) -> Window{
            let parent_window: winit::raw_window_handle::WindowHandle = parent.window_handle().unwrap();
            
            let mut window_attributes = Window::default_attributes()
            .with_title("title")
            .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
            .with_position(Position::Logical(LogicalPosition::new(0.0, 0.0)))
            .with_visible(true);
            
            //They make a unsafe function call here I wonder why? "window_attributes = unsafe { window_attributes.with_parent_window(Some(parent)) };"
            //Should look into why that function is unsafe

            return  event_loop.create_window(window_attributes).unwrap();
        }
    }
}
*/
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
    let event_loop = EventLoop::new().unwrap();
    let mut vulkan_app = TriangleAppTest::initialize(&event_loop);
    
    vulkan_app.create_window(event_loop, None);
    
    //vulkan_app.main_loop();
}