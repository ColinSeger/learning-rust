//#![allow(unused)] //REMOVE LATER IMPORTANT
//extern crate winit;

mod open_gl_context;
mod window_context;

//use open_gl_context::gl_window;

enum Renderer{
    //OpenGL,
    Vulkan
}

fn main() {
    let render: Renderer = Renderer::Vulkan;
    //let wind = window_context::window_context::start_window().expect("Window error");
    let wib = window_context::test::test();

    match render {
        //Renderer::OpenGL => println!("OpenGl"),
        Renderer::Vulkan => println!("Vulkan")
    }

}