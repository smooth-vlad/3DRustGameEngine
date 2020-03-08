#[macro_use]
extern crate glium;
extern crate wavefront_obj;

use glium::{glutin, Surface};
use glutin::event;
use wavefront_obj::obj;

use std::fs;
use std::rc::Rc;

mod game_engine;

use game_engine::object3d::Object3D;
use game_engine::math;
use game_engine::vector3::Vector3;
use game_engine::renderer::Renderer;
use game_engine::mesh::Mesh;
use game_engine::material::Material;
use game_engine::color::Color;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let s = event_loop.available_monitors().next().unwrap();
    let wb = glutin::window::WindowBuilder::new().with_fullscreen(Some(glutin::window::Fullscreen::Borderless(s)));
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex_shader_src = fs::read_to_string("assets/shaders/vertex_shader.glsl").unwrap();
    let fragment_shader_src = fs::read_to_string("assets/shaders/fragment_shader.glsl").unwrap();

    let program = Rc::new(glium::Program::from_source(&display,
        &vertex_shader_src.to_string(),
        &fragment_shader_src.to_string(), None).unwrap());

    // objects

    let mut board = {
        let object = {
            let source = fs::read_to_string("assets/models/board.obj").unwrap();
            obj::parse(source).unwrap()
        };
        let mut mesh = Mesh::new(&object.objects[0], &display);

        let material1 = Material {
            albedo: Color::new(0.075, 0.04, 0.01, 1.0),
            shader: program.clone(),
        };
        let material2 = Material {
            albedo: Color::new(1.0, 1.0, 1.0, 1.0),
            shader: program.clone(),
        };
        mesh.materials = vec!(material1.clone(), material2, material1);
        Object3D::new(mesh)
    };
    board.mesh.transform.scale(Vector3::fill(0.1));
    board.mesh.transform.translate(Vector3::new(0.0, -0.2, 0.0));

    let mut rook = {
        let object = {
            let source = fs::read_to_string("assets/models/rook.obj").unwrap();
            obj::parse(source).unwrap()
        };
        let mut mesh = Mesh::new(&object.objects[0], &display);
        mesh.materials = vec!(Material {
            albedo: Color::new(0.7, 0.3, 0.1, 1.0),
            shader: program.clone(),
        });
        Object3D::new(mesh)
    };
    rook.mesh.transform.scale(Vector3::fill(0.1));

    // variables

    let mut angle: f32 = 0.0;
    let speed: f32 = 0.5;

    let mut movement_buttons = [false; 4];
    let mut view_pos = Vector3::new(0.0, 1.5, -2.0);

    let mut yaw = -90.0;
    let mut pitch = 0.0;

    let mut elapsed_time: f32 = 0.0;

    // =======================
    // ====== loop ===========
    // =======================

    event_loop.run(move |event, _, control_flow| {

        let frame_time = std::time::Instant::now();

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                event::WindowEvent::KeyboardInput { input: e, .. } => {
                    match e {
                        event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::W), .. } => movement_buttons[0] = e.state == event::ElementState::Pressed,
                        event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::S), .. } => movement_buttons[1] = e.state == event::ElementState::Pressed,
                        event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::A), .. } => movement_buttons[2] = e.state == event::ElementState::Pressed,
                        event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::D), .. } => movement_buttons[3] = e.state == event::ElementState::Pressed,
                        _ => (),
                    }
                },
                _ => return,
            },
            event::Event::NewEvents(cause) => match cause {
                event::StartCause::Init => (),
                event::StartCause::Poll => (),
                _ => return,
            },
            _ => return,
        }

        // update

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        if movement_buttons[0] { pitch += elapsed_time * 10.0 };
        if movement_buttons[1] { pitch -= elapsed_time * 10.0 };
        if movement_buttons[2] { yaw -= elapsed_time * 10.0 };
        if movement_buttons[3] { yaw += elapsed_time * 10.0 };

        if pitch > 89.0 {
            pitch =  89.0;
        }
        if pitch < -89.0 {
            pitch = -89.0;
        }

        let pitch = pitch * (std::f32::consts::PI / 180.0);
        let yaw = yaw * (std::f32::consts::PI / 180.0);

        let direction = Vector3::new(
            pitch.cos() * yaw.cos(),
            pitch.sin(),
            -pitch.cos() * yaw.sin(),
        ).normalized();

        let view = math::view_matrix(
            view_pos,
            direction,
            Vector3::new(0.0, 1.0, 0.0));

        let frame_size = display.get_framebuffer_dimensions();
        let mut renderer = Renderer::new(&display,
            [0.0, 0.0, 0.0],
            [1.4, 0.4, -0.7f32],
            view,
            math::perspective_matrix(frame_size, 3.141592 / 3.0, 1024.0, 0.1),
            params,
        );

        rook.mesh.transform.set_position(Vector3::new(angle.sin(), 0.0, angle.cos())
            * rook.mesh.transform.get_scale().z * 3.0);
        angle += speed * elapsed_time;

        // draw

        renderer.clear(Color::new(0.02, 0.02, 0.02, 1.0));
        renderer.draw(&rook.mesh);
        renderer.draw(&board.mesh);
        renderer.show();

        elapsed_time = frame_time.elapsed().as_secs_f32();
    });
}