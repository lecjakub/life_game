#[macro_use]
extern crate glium;
extern crate image;




#[derive(Copy,Clone)]
struct Vertex{
    position:[f32;2],
    tex_coords:[f32;2],
}

implement_vertex!(Vertex, position, tex_coords);

impl Vertex{
    
    //func new return new vertex
    fn new(pos:[f32;2],t_coords:[f32;2]) -> Vertex{
        Vertex{
            position: pos,
            tex_coords: t_coords,
        }
    }
}

fn main() {
    use glium::{glutin,Surface};
    use std::io::Cursor;
    // 1. The **winit::EventsLoop** for handling events.
    let mut events_loop = glium::glutin::EventsLoop::new();
    // 2. Parameters for building the Window.
    let wb = glium::glutin::WindowBuilder::new()
        .with_dimensions(glium::glutin::dpi::LogicalSize::new(1024f64, 768f64))
        .with_title("Hello world");
    // 3. Parameters for building the OpenGL context.
    let cb = glium::glutin::ContextBuilder::new();
    // 4. Build the Display with the given window and OpenGL context parameters and register the
    //    window with the events_loop.
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();



    //loading image
    let image = image::load(Cursor::new(&include_bytes!("assets/morty.png")[..]),
                            image::PNG).expect("image cannot be loaded").to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(&display,image).unwrap();



    //triangle variables
    let vertex1 = Vertex::new([-0.5,-0.5], [0.0,0.0]);
    let vertex2 = Vertex::new([0.0,0.5], [0.0,1.0]);
    let vertex3 = Vertex::new([0.5,-0.25], [1.0,0.0]);
    let shape = vec![vertex1,vertex2,vertex3];

    //creating access to video card memory;
    let vertex_buffer = glium::VertexBuffer::new(&display,&shape).unwrap();


    //creating indicies
    let indicies = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        
        in vec2 position;
        in vec2 tex_coords;

        out vec2 v_tex_coords;


        void main(){
            v_tex_coords = tex_coords;
            gl_Position = vec4(position, 0.0,1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        
        in vec2 v_tex_coords;
        
        out vec4 color;

        uniform sampler2D tex;


        void main(){
            color = texture(tex,v_tex_coords);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t: f32 = -0.5;
    let mut closed = false;
    while !closed {
        t += 0.002;
        if t > 0.5 {
            t = -0.5;
        }

        let mut target = display.draw();
        target.clear_color(0.0,0.5,1.0,1.0);


        let uniforms = glium::uniform! {
            rot:[
                [ t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
            tex: &texture,
            
        };
        target.draw(&vertex_buffer, &indicies, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });
    }

}