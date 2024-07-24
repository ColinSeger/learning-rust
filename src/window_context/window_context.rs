extern crate winit;

use std::{
    collections::HashMap, 
    error::Error, 
    sync::Arc,//Some heap allocated thing (Should read more)
    num::NonZeroU32,
};

use rwh_06::{DisplayHandle, HasDisplayHandle};

use softbuffer::{Context, Surface};

use winit::{
    application::ApplicationHandler, dpi::{
        LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize, Position
    }, error::EventLoopError, event::{
        ElementState,
        KeyEvent, 
        WindowEvent
    }, event_loop::{
        self, ActiveEventLoop, EventLoop
    }, keyboard::ModifiersState, raw_window_handle::HasWindowHandle, window::{
        self, CursorGrabMode, ResizeDirection, Theme, Window, WindowId
    }
};
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const BORDER_SIZE: f64 = 20.;

pub fn start_window() -> Result<(), EventLoopError>{
    println!("Window");

    let event_loop= EventLoop::new().expect("event_loop Failed");

    let mut app = Application::new(&event_loop);

    return event_loop.run_app(&mut app);
}

struct Application{//This is probably the application itself
    //instance: Option<Arc<Instance>>, Implement later
    //parent_window_id: Option<WindowId>,
    windows: HashMap<WindowId, WindowState>,
    context: Option<Context<DisplayHandle<'static>>>,
    //window: Window

}

impl Application {

    fn new(event_loop: &EventLoop<()>) -> Self {
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

    fn create_a_window
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

        let window_state = WindowState::new(self, window)?;
        let window_id = window_state.window.id();
        //info!("Created new window with id={window_id:?}");
        self.windows.insert(window_id, window_state);
        Ok(window_id)
    }
}

struct WindowState{
    // IME input.
    ime: bool, //Wtf does ime stand for? am I dumb? Imput manager?

    //A render surface aka what i will draw to later
    surface: Surface<DisplayHandle<'static>, Arc<Window>>,
    //I apparantly MUST drop surface before window

    //Actual window
    window: Arc<Window>,

    //Some kind of theme for the drawing window
    theme: Theme,

    //The cursor position over the window
    cursor_position: Option<PhysicalPosition<f64>>,

    //Window modifiers state?
    modifiers: ModifiersState,

    //Occlution state of the window?
    occluded: bool,

    //Current cursor grab mode
    cursor_grab: CursorGrabMode,

    //zoom into window
    zoom: f64,

    //Window Rotation??? Research this
    rotated: f32,

    //window pan
    panned: PhysicalPosition<f32>,

    //Cursor States
    named_idx: usize,
    custom_idx: usize,
    cursor_hidden: bool,
}

impl ApplicationHandler for Application {
        // Required methods
        fn resumed(&mut self, event_loop: &ActiveEventLoop){
            println!("resumed");
        }
        fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            window_id: WindowId,
            event: WindowEvent,
        ){

        }
}

impl WindowState {
    fn new(app: &Application, window: Window) -> Result<Self, Box<dyn Error>> {
        
        let window: Arc<Window> = Arc::new(window);

        let surface = Surface::new(app.context.as_ref().unwrap(), Arc::clone(&window))?;

        let theme: Theme = window.theme().unwrap_or(Theme::Dark);

        let named_idx = 0;

        //Ime out of some kind of box?
        let ime: bool = true;
        window.set_ime_allowed(ime);

        let size: winit::dpi::PhysicalSize<u32> = window.inner_size();

        let mut state = Self {
            #[cfg(macos_platform)]
            option_as_alt: window.option_as_alt(),
            custom_idx: 0,
            cursor_grab: CursorGrabMode::None,
            named_idx,
            #[cfg(not(any(android_platform, ios_platform)))]
            surface,
            window,
            theme,
            ime,
            cursor_position: Default::default(),
            cursor_hidden: Default::default(),
            modifiers: Default::default(),
            occluded: Default::default(),
            rotated: Default::default(),
            panned: Default::default(),
            zoom: Default::default(),
        };

        state.resize(size);
        Ok(state)
    }

    fn toggle_maximize(&self) {
        let maximized = self.window.is_maximized();
        self.window.set_maximized(!maximized);
    }

    pub fn minimize(&mut self) {
        self.window.set_minimized(true);
    }

    //Cursor stuff
    pub fn cursor_moved(&mut self, position: PhysicalPosition<f64>) {
        self.cursor_position = Some(position);
        if self.ime {
            self.window.set_ime_cursor_area(position, PhysicalSize::new(20, 20));
        }
    }
    pub fn cursor_left(&mut self) {
        self.cursor_position = None;
    }


    /// Drag the window.
    fn drag_window(&self) {
        if let Err(err) = self.window.drag_window() {
            //info!("Error starting window drag: {err}");
        } else {
            //info!("Dragging window Window={:?}", self.window.id());
        }
    }
    /// Drag-resize the window.
    fn drag_resize_window(&self) {
        let position = match self.cursor_position {
            Some(position) => position,
            None => {
                //info!("Drag-resize requires cursor to be inside the window");
                return;
            },
        };

        let win_size = self.window.inner_size();
        let border_size = BORDER_SIZE * self.window.scale_factor();

        let x_direction = if position.x < border_size {
            ResizeDirection::West
        } else if position.x > (win_size.width as f64 - border_size) {
            ResizeDirection::East
        } else {
            // Use arbitrary direction instead of None for simplicity.
            ResizeDirection::SouthEast
        };

        let y_direction = if position.y < border_size {
            ResizeDirection::North
        } else if position.y > (win_size.height as f64 - border_size) {
            ResizeDirection::South
        } else {
            // Use arbitrary direction instead of None for simplicity.
            ResizeDirection::SouthEast
        };

        let direction = match (x_direction, y_direction) {
            (ResizeDirection::West, ResizeDirection::North) => ResizeDirection::NorthWest,
            (ResizeDirection::West, ResizeDirection::South) => ResizeDirection::SouthWest,
            (ResizeDirection::West, _) => ResizeDirection::West,
            (ResizeDirection::East, ResizeDirection::North) => ResizeDirection::NorthEast,
            (ResizeDirection::East, ResizeDirection::South) => ResizeDirection::SouthEast,
            (ResizeDirection::East, _) => ResizeDirection::East,
            (_, ResizeDirection::South) => ResizeDirection::South,
            (_, ResizeDirection::North) => ResizeDirection::North,
            _ => return,
        };

        if let Err(err) = self.window.drag_resize_window(direction) {
            //info!("Error starting window drag-resize: {err}");
        } else {
            //info!("Drag-resizing window Window={:?}", self.window.id());
        }
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        //info!("Resized to {size:?}");
        #[cfg(not(any(android_platform, ios_platform)))]
        {
            let (width, height) = match (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
            {
                (Some(width), Some(height)) => (width, height),
                _ => return,
            };
            self.surface.resize(width, height).expect("failed to resize inner buffer");
        }
        self.window.request_redraw();
    }


    fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        if self.occluded {
            //info!("Skipping drawing occluded window={:?}", self.window.id());
            return Ok(());
        }

        const WHITE: u32 = 0xffffffff;
        const DARK_GRAY: u32 = 0xff181818;

        let color = match self.theme {
            Theme::Light => WHITE,
            Theme::Dark => DARK_GRAY,
        };

        let mut buffer = self.surface.buffer_mut()?;
        buffer.fill(color);
        self.window.pre_present_notify();
        buffer.present()?;
        Ok(())
    }
}