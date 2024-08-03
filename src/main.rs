use gl::{ARRAY_BUFFER, COLOR_BUFFER_BIT, COMPILE_STATUS, FRAGMENT_SHADER, LINK_STATUS, STATIC_DRAW, TRIANGLES, 
VERTEX_SHADER, FLOAT, FALSE};
    
use glfw::{Action, Context, Key};

use std::mem::{size_of_val, size_of};

fn main() {
    unsafe{
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut window, events) = glfw.create_window(700, 700, "Triángulo",  glfw::WindowMode::Windowed) //glfw::WindowMode::FullScreen()
        .expect("Failed to create GLFW window.");

        window.make_current();
        window.set_key_polling(true);

        gl_loader::init_gl();
        gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);

        let vertex_shader_source = "
            #version 330 core
            layout (location = 0) in vec3 aPos;
    
            void main()
            {
                gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
            }
        ";
    
        let vertex_shader = gl::CreateShader(VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, &(vertex_shader_source.as_bytes().as_ptr().cast()), &(vertex_shader_source.len().try_into().unwrap()));
        gl::CompileShader(vertex_shader);
    
        let mut success = 0;
        gl::GetShaderiv(vertex_shader, COMPILE_STATUS, &mut success);

        if success == 0 {
            println!("Error at vertex shader");
        }

        let fragment_shader_source = "
            #version 330 core
            out vec4 FragColor;
    
            void main()
            {
               FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
            }
        ";

        let fragment_shader = gl::CreateShader(FRAGMENT_SHADER);
        gl::ShaderSource(fragment_shader, 1, &(fragment_shader_source.as_bytes().as_ptr().cast()), &(fragment_shader_source.len().try_into().unwrap()));
        gl::CompileShader(fragment_shader);
    
        gl::GetShaderiv(fragment_shader, COMPILE_STATUS, &mut success);

        if success == 0 {
            println!("Error at fragment shader");
        }

        let shader_program = gl::CreateProgram();
    
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
    
        gl::GetProgramiv(shader_program, LINK_STATUS, &mut success);
    
        if success == 0 {
            println!("Error in shader prorgram");
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        type Vertex = [f32; 3];
        let vertices: [Vertex; 3] = [
            [-0.5, -0.5, 0.0],     //left                  
            [0.5, -0.5, 0.0],       //right
            [0.0, 0.5, 0.0],       //top

            /*[-0.55, -0.5, 0.0],   //este es el primero
            [-0.35, -0.5, 0.0],      
            [-0.55, 0.5, 0.0], 

            [-0.35, 0.5, 0.0],
            [-0.35, -0.5, 0.0],      
            [-0.55, 0.5, 0.0],

            [-0.35, 0.35, 0.0],    
            [0.5, 0.35, 0.0],      
            [-0.35, 0.5, 0.0],    

            [0.3, 0.5, 0.0],    
            [0.5, 0.35, 0.0],      
            [-0.35, 0.5, 0.0], */
        ];

        let mut vbo = 0;
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(vao);

        gl::BindBuffer(ARRAY_BUFFER, vbo);
        gl::BufferData(ARRAY_BUFFER, size_of_val(&vertices) as isize, vertices.as_ptr().cast(), STATIC_DRAW);

        gl::VertexAttribPointer(
            0,
            3,
            FLOAT,
            FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(ARRAY_BUFFER, 0); /////

        gl::BindVertexArray(0); 

        while !window.should_close() {
    
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(COLOR_BUFFER_BIT);
    
            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(TRIANGLES, 0, 3);
            
            window.swap_buffers();    /////

            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                handle_window_event(&mut window, event);
            }
        }
        gl_loader::end_gl();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}