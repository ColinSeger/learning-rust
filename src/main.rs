extern crate vulkano;
extern crate winit;

mod open_gl_context;
mod vulkan_context;

//use open_gl_context::gl_window;

enum Renderer{
    //OpenGL,
    Vulkan
}

fn main() {
    let render: Renderer = Renderer::Vulkan;
    vulkan_context::inital_vulkan_test::vulkan_instance();
    match render {
        //Renderer::OpenGL => println!("OpenGl"),
        Renderer::Vulkan => println!("Vulkan")
    }

}