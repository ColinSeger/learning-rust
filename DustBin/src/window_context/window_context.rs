extern crate winit;

use std::{
    collections::HashMap, 
    error::Error, 
    sync::Arc,//Some heap allocated thing (Should read more)
    num::NonZeroU32,
    fmt,
    fmt::Debug,
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
        self, ActiveEventLoop, EventLoop,
    }, keyboard::ModifiersState, raw_window_handle::HasWindowHandle, window::{
        self, CursorGrabMode, ResizeDirection, Theme, Window, WindowId
    },
    event::{
        MouseScrollDelta
    },
    window::{
        Fullscreen,
        CustomCursor
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
    custom_cursors: Vec<CustomCursor>,
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
        let custom_cursors = vec![
            event_loop.create_custom_cursor(decode_cursor(include_bytes!("data/cross.png"))),
            event_loop.create_custom_cursor(decode_cursor(include_bytes!("data/cross2.png"))),
            event_loop.create_custom_cursor(decode_cursor(include_bytes!("data/gradient.png"))),
        ];

        Self{
            custom_cursors,
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
    fn handle_action(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, action: Action) {
        // let cursor_position = self.cursor_position;
        let window = self.windows.get_mut(&window_id).unwrap();
        //info!("Executing action: {action:?}");
        match action {
            Action::CloseWindow => {
                let _ = self.windows.remove(&window_id);
            },
            Action::CreateNewWindow => {
                #[cfg(any(x11_platform, wayland_platform))]
                if let Err(err) = window.window.request_activation_token() {
                    info!("Failed to get activation token: {err}");
                } else {
                    return;
                }

                if let Err(err) = self.create_a_window(event_loop, None) {
                    //error!("Error creating new window: {err}");
                }
            },
            Action::ToggleResizeIncrements => window.toggle_resize_increments(),
            Action::ToggleCursorVisibility => window.toggle_cursor_visibility(),
            Action::ToggleResizable => window.toggle_resizable(),
            Action::ToggleDecorations => window.toggle_decorations(),
            Action::ToggleFullscreen => window.toggle_fullscreen(),
            Action::ToggleMaximize => window.toggle_maximize(),
            Action::ToggleImeInput => window.toggle_ime(),
            Action::Minimize => window.minimize(),
            Action::NextCursor => window.next_cursor(),
            Action::NextCustomCursor => window.next_custom_cursor(&self.custom_cursors),
            #[cfg(web_platform)]
            Action::UrlCustomCursor => window.url_custom_cursor(event_loop),
            #[cfg(web_platform)]
            Action::AnimationCustomCursor => {
                window.animation_custom_cursor(event_loop, &self.custom_cursors)
            },
            Action::CycleCursorGrab => window.cycle_cursor_grab(),
            Action::DragWindow => window.drag_window(),
            Action::DragResizeWindow => window.drag_resize_window(),
            Action::ShowWindowMenu => window.show_menu(),
            Action::PrintHelp => self.print_help(),
            #[cfg(macos_platform)]
            Action::CycleOptionAsAlt => window.cycle_option_as_alt(),
            Action::SetTheme(theme) => {
                window.window.set_theme(theme);
                // Get the resulting current theme to draw with
                let actual_theme = theme.or_else(|| window.window.theme()).unwrap_or(Theme::Dark);
                window.set_draw_theme(actual_theme);
            },
            #[cfg(macos_platform)]
            Action::CreateNewTab => {
                let tab_id = window.window.tabbing_identifier();
                if let Err(err) = self.create_window(event_loop, Some(tab_id)) {
                    error!("Error creating new window: {err}");
                }
            },
            Action::RequestResize => window.swap_dimensions(),
        }
    }
    fn print_help(&self) {
       /* //info!("Keyboard bindings:");
        for binding in KEY_BINDINGS {
            /*info!(
                "{}{:<10} - {} ({})",
                modifiers_to_string(binding.mods),
                binding.trigger,
                binding.action,
                binding.action.help(),
            ); */
        }
        //info!("Mouse bindings:");
        for binding in MOUSE_BINDINGS {
            info!(
                "{}{:<10} - {} ({})",
                modifiers_to_string(binding.mods),
                mouse_button_to_string(binding.trigger),
                binding.action,
                binding.action.help(),
            );
        }*/
    }
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
            let window = match self.windows.get_mut(&window_id) {
                Some(window) => window,
                None => return,
            };
    
            match event {
                WindowEvent::Resized(size) => {
                    window.resize(size);
                },
                WindowEvent::Focused(focused) => {
                    if focused {
                        //info!("Window={window_id:?} focused");
                    } else {
                        //info!("Window={window_id:?} unfocused");
                    }
                },
                WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                    //info!("Window={window_id:?} changed scale to {scale_factor}");
                },
                WindowEvent::ThemeChanged(theme) => {
                    //info!("Theme changed to {theme:?}");
                    window.set_draw_theme(theme);
                },
                WindowEvent::RedrawRequested => {
                    if let Err(err) = window.draw() {
                        //error!("Error drawing window: {err}");
                    }
                },
                WindowEvent::Occluded(occluded) => {
                    window.set_occluded(occluded);
                },
                WindowEvent::CloseRequested => {
                    //info!("Closing Window={window_id:?}");
                    self.windows.remove(&window_id);
                },
                WindowEvent::ModifiersChanged(modifiers) => {
                    window.modifiers = modifiers.state();
                    //info!("Modifiers changed to {:?}", window.modifiers);
                },
                WindowEvent::MouseWheel { delta, .. } => match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        //info!("Mouse wheel Line Delta: ({x},{y})");
                    },
                    MouseScrollDelta::PixelDelta(px) => {
                        //info!("Mouse wheel Pixel Delta: ({},{})", px.x, px.y);
                    },
                },
                WindowEvent::KeyboardInput { event, is_synthetic: false, .. } => {
                    let mods = window.modifiers;
    
                    // Dispatch actions only on press.
                    if event.state.is_pressed() {
                        let action = if let Key::Character(ch) = event.logical_key.as_ref() {
                            Self::process_key_binding(&ch.to_uppercase(), &mods)
                        } else {
                            None
                        };
    
                        if let Some(action) = action {
                            self.handle_action(event_loop, window_id, action);
                        }
                    }
                },
                WindowEvent::MouseInput { button, state, .. } => {
                    let mods = window.modifiers;
                    if let Some(action) =
                        state.is_pressed().then(|| Self::process_mouse_binding(button, &mods)).flatten()
                    {
                        self.handle_action(event_loop, window_id, action);
                    }
                },
                WindowEvent::CursorLeft { .. } => {
                    //info!("Cursor left Window={window_id:?}");
                    window.cursor_left();
                },
                WindowEvent::CursorMoved { position, .. } => {
                    //info!("Moved cursor to {position:?}");
                    window.cursor_moved(position);
                },
                WindowEvent::ActivationTokenDone { token: _token, .. } => {
                    #[cfg(any(x11_platform, wayland_platform))]
                    {
                        startup_notify::set_activation_token_env(_token);
                        if let Err(err) = self.create_window(event_loop, None) {
                            error!("Error creating new window: {err}");
                        }
                    }
                },
                WindowEvent::Ime(event) => match event {
                    Ime::Enabled => info!("IME enabled for Window={window_id:?}"),
                    Ime::Preedit(text, caret_pos) => {
                        //info!("Preedit: {}, with caret at {:?}", text, caret_pos);
                    },
                    Ime::Commit(text) => {
                        //info!("Committed: {}", text);
                    },
                    Ime::Disabled => info!("IME disabled for Window={window_id:?}"),
                },
                WindowEvent::PinchGesture { delta, .. } => {
                    window.zoom += delta;
                    let zoom = window.zoom;
                    if delta > 0.0 {
                        //info!("Zoomed in {delta:.5} (now: {zoom:.5})");
                    } else {
                        //info!("Zoomed out {delta:.5} (now: {zoom:.5})");
                    }
                },
                WindowEvent::RotationGesture { delta, .. } => {
                    window.rotated += delta;
                    let rotated = window.rotated;
                    if delta > 0.0 {
                        //info!("Rotated counterclockwise {delta:.5} (now: {rotated:.5})");
                    } else {
                        //info!("Rotated clockwise {delta:.5} (now: {rotated:.5})");
                    }
                },
                WindowEvent::PanGesture { delta, phase, .. } => {
                    window.panned.x += delta.x;
                    window.panned.y += delta.y;
                    //info!("Panned ({delta:?})) (now: {:?}), {phase:?}", window.panned);
                },
                WindowEvent::DoubleTapGesture { .. } => {
                    //info!("Smart zoom");
                },
                WindowEvent::TouchpadPressure { .. }
                | WindowEvent::HoveredFileCancelled
                | WindowEvent::KeyboardInput { .. }
                | WindowEvent::CursorEntered { .. }
                | WindowEvent::AxisMotion { .. }
                | WindowEvent::DroppedFile(_)
                | WindowEvent::HoveredFile(_)
                | WindowEvent::Destroyed
                | WindowEvent::Touch(_)
                | WindowEvent::Moved(_) => (),
            }
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

    pub fn toggle_ime(&mut self) {
        self.ime = !self.ime;
        self.window.set_ime_allowed(self.ime);
        if let Some(position) = self.ime.then_some(self.cursor_position).flatten() {
            self.window.set_ime_cursor_area(position, PhysicalSize::new(20, 20));
        }
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


    /// Toggle window decorations.
    fn toggle_decorations(&self) {
        let decorated = self.window.is_decorated();
        self.window.set_decorations(!decorated);
    }

    /// Toggle window resizable state.
    fn toggle_resizable(&self) {
        let resizable = self.window.is_resizable();
        self.window.set_resizable(!resizable);
    }

    /// Toggle cursor visibility
    fn toggle_cursor_visibility(&mut self) {
        self.cursor_hidden = !self.cursor_hidden;
        self.window.set_cursor_visible(!self.cursor_hidden);
    }

    /// Toggle resize increments on a window.
    fn toggle_resize_increments(&mut self) {
        let new_increments = match self.window.resize_increments() {
            Some(_) => None,
            None => Some(LogicalSize::new(25.0, 25.0)),
        };
        //info!("Had increments: {}", new_increments.is_none());
        self.window.set_resize_increments(new_increments);
    }

    /// Toggle fullscreen.
    fn toggle_fullscreen(&self) {
        let fullscreen = if self.window.fullscreen().is_some() {
            None
        } else {
            Some(Fullscreen::Borderless(None))
        };

        self.window.set_fullscreen(fullscreen);
    }

    /// Cycle through the grab modes ignoring errors.
    fn cycle_cursor_grab(&mut self) {
        self.cursor_grab = match self.cursor_grab {
            CursorGrabMode::None => CursorGrabMode::Confined,
            CursorGrabMode::Confined => CursorGrabMode::Locked,
            CursorGrabMode::Locked => CursorGrabMode::None,
        };
        //info!("Changing cursor grab mode to {:?}", self.cursor_grab);
        if let Err(err) = self.window.set_cursor_grab(self.cursor_grab) {
            //error!("Error setting cursor grab: {err}");
        }
    }

    /// Change the theme that things are drawn in.
    fn set_draw_theme(&mut self, theme: Theme) {
        self.theme = theme;
        self.window.request_redraw();
    }

    /// Show window menu.
    fn show_menu(&self) {
        if let Some(position) = self.cursor_position {
            self.window.show_window_menu(position);
        }
    }


    /// Cycle through the grab modes ignoring errors.
    fn cycle_cursor_grab(&mut self) {
        self.cursor_grab = match self.cursor_grab {
            CursorGrabMode::None => CursorGrabMode::Confined,
            CursorGrabMode::Confined => CursorGrabMode::Locked,
            CursorGrabMode::Locked => CursorGrabMode::None,
        };
        info!("Changing cursor grab mode to {:?}", self.cursor_grab);
        if let Err(err) = self.window.set_cursor_grab(self.cursor_grab) {
            error!("Error setting cursor grab: {err}");
        }
    }

    #[cfg(macos_platform)]
    fn cycle_option_as_alt(&mut self) {
        self.option_as_alt = match self.option_as_alt {
            OptionAsAlt::None => OptionAsAlt::OnlyLeft,
            OptionAsAlt::OnlyLeft => OptionAsAlt::OnlyRight,
            OptionAsAlt::OnlyRight => OptionAsAlt::Both,
            OptionAsAlt::Both => OptionAsAlt::None,
        };
        info!("Setting option as alt {:?}", self.option_as_alt);
        self.window.set_option_as_alt(self.option_as_alt);
    }

    /// Swap the window dimensions with `request_inner_size`.
    fn swap_dimensions(&mut self) {
        let old_inner_size = self.window.inner_size();
        let mut inner_size = old_inner_size;

        mem::swap(&mut inner_size.width, &mut inner_size.height);
        info!("Requesting resize from {old_inner_size:?} to {inner_size:?}");

        if let Some(new_inner_size) = self.window.request_inner_size(inner_size) {
            if old_inner_size == new_inner_size {
                info!("Inner size change got ignored");
            } else {
                self.resize(new_inner_size);
            }
        } else {
            info!("Request inner size is asynchronous");
        }
    }

    /// Pick the next cursor.
    fn next_cursor(&mut self) {
        self.named_idx = (self.named_idx + 1) % CURSORS.len();
        info!("Setting cursor to \"{:?}\"", CURSORS[self.named_idx]);
        self.window.set_cursor(Cursor::Icon(CURSORS[self.named_idx]));
    }

    /// Pick the next custom cursor.
    fn next_custom_cursor(&mut self, custom_cursors: &[CustomCursor]) {
        self.custom_idx = (self.custom_idx + 1) % custom_cursors.len();
        let cursor = Cursor::Custom(custom_cursors[self.custom_idx].clone());
        self.window.set_cursor(cursor);
    }

    /// Custom cursor from an URL.
    #[cfg(web_platform)]
    fn url_custom_cursor(&mut self, event_loop: &ActiveEventLoop) {
        let cursor = event_loop.create_custom_cursor(url_custom_cursor());

        self.window.set_cursor(cursor);
    }

    /// Custom cursor from a URL.
    #[cfg(web_platform)]
    fn animation_custom_cursor(
        &mut self,
        event_loop: &ActiveEventLoop,
        custom_cursors: &[CustomCursor],
    ) {
        use std::time::Duration;

        use winit::platform::web::CustomCursorExtWeb;

        let cursors = vec![
            custom_cursors[0].clone(),
            custom_cursors[1].clone(),
            event_loop.create_custom_cursor(url_custom_cursor()),
        ];
        let cursor = CustomCursor::from_animation(Duration::from_secs(3), cursors).unwrap();
        let cursor = event_loop.create_custom_cursor(cursor);

        self.window.set_cursor(cursor);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    CloseWindow,
    ToggleCursorVisibility,
    CreateNewWindow,
    ToggleResizeIncrements,
    ToggleImeInput,
    ToggleDecorations,
    ToggleResizable,
    ToggleFullscreen,
    ToggleMaximize,
    Minimize,
    NextCursor,
    NextCustomCursor,
    #[cfg(web_platform)]
    UrlCustomCursor,
    #[cfg(web_platform)]
    AnimationCustomCursor,
    CycleCursorGrab,
    PrintHelp,
    DragWindow,
    DragResizeWindow,
    ShowWindowMenu,
    #[cfg(macos_platform)]
    CycleOptionAsAlt,
    SetTheme(Option<Theme>),
    #[cfg(macos_platform)]
    CreateNewTab,
    RequestResize,
}

impl Action {
    fn help(&self) -> &'static str {
        match self {
            Action::CloseWindow => "Close window",
            Action::ToggleCursorVisibility => "Hide cursor",
            Action::CreateNewWindow => "Create new window",
            Action::ToggleImeInput => "Toggle IME input",
            Action::ToggleDecorations => "Toggle decorations",
            Action::ToggleResizable => "Toggle window resizable state",
            Action::ToggleFullscreen => "Toggle fullscreen",
            Action::ToggleMaximize => "Maximize",
            Action::Minimize => "Minimize",
            Action::ToggleResizeIncrements => "Use resize increments when resizing window",
            Action::NextCursor => "Advance the cursor to the next value",
            Action::NextCustomCursor => "Advance custom cursor to the next value",
            #[cfg(web_platform)]
            Action::UrlCustomCursor => "Custom cursor from an URL",
            #[cfg(web_platform)]
            Action::AnimationCustomCursor => "Custom cursor from an animation",
            Action::CycleCursorGrab => "Cycle through cursor grab mode",
            Action::PrintHelp => "Print help",
            Action::DragWindow => "Start window drag",
            Action::DragResizeWindow => "Start window drag-resize",
            Action::ShowWindowMenu => "Show window menu",
            #[cfg(macos_platform)]
            Action::CycleOptionAsAlt => "Cycle option as alt mode",
            Action::SetTheme(None) => "Change to the system theme",
            Action::SetTheme(Some(Theme::Light)) => "Change to a light theme",
            Action::SetTheme(Some(Theme::Dark)) => "Change to a dark theme",
            #[cfg(macos_platform)]
            Action::CreateNewTab => "Create new tab",
            Action::RequestResize => "Request a resize",
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}
fn decode_cursor(bytes: &[u8]) -> CustomCursorSource {
    let img = image::load_from_memory(bytes).unwrap().to_rgba8();
    let samples = img.into_flat_samples();
    let (_, w, h) = samples.extents();
    let (w, h) = (w as u16, h as u16);
    CustomCursor::from_rgba(samples.samples, w, h, w / 2, h / 2).unwrap()
}