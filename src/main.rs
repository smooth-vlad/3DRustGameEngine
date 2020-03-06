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

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let s = event_loop.available_monitors().next().unwrap();
    let wb = glutin::window::WindowBuilder::new().with_fullscreen(Some(glutin::window::Fullscreen::Borderless(s)));
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex_shader_src = fs::read_to_string("assets/shaders/vertex_shader.glsl").unwrap();
    let fragment_shader_src = fs::read_to_string("assets/shaders/fragment_shader.glsl").unwrap();

    let program = Rc::new(glium::Program::from_source(&display, &vertex_shader_src.to_string(),
        &fragment_shader_src.to_string(), None).unwrap());

    // objects

    let mut figure = {
        let object = {
            let source = fs::read_to_string("assets/models/king.obj").unwrap();
            obj::parse(source).unwrap()
        };
        Object3D::new(&object.objects[0], &display, program.clone())
    };
    figure.mesh.transform.scale(Vector3::fill(0.1));
    figure.mesh.material.albedo = [0.3, 0.1, 1.0];

    let mut board = {
        let object = {
            let source = fs::read_to_string("assets/models/board.obj").unwrap();
            obj::parse(source).unwrap()
        };
        Object3D::new(&object.objects[0], &display, program.clone())
    };
    board.mesh.transform.scale(Vector3::fill(0.1));
    board.mesh.transform.translate(Vector3::new(0.0, -0.5, 0.0));
    board.mesh.material.albedo = [0.8, 0.8, 0.8];

    let mut figure2 = {
        let object = {
            let source = fs::read_to_string("assets/models/knight.obj").unwrap();
            obj::parse(source).unwrap()
        };
        Object3D::new(&object.objects[0], &display, program.clone())
    };
    figure2.mesh.transform.scale(Vector3::fill(0.1));
    figure2.mesh.material.albedo = [0.7, 0.6, 0.1];

    let mut figure3 = {
        let object = {
            let source = fs::read_to_string("assets/models/rook.obj").unwrap();
            obj::parse(source).unwrap()
        };
        Object3D::new(&object.objects[0], &display, program.clone())
    };
    figure3.mesh.transform.scale(Vector3::fill(0.1));
    figure3.mesh.material.albedo = [0.7, 0.3, 0.1];

    // variables

    let mut angle: f32 = 0.0;
    let speed: f32 = 0.5;

    let mut angle2: f32 = 0.0;
    let speed2: f32 = 0.25;

    // =======================
    // ====== loop ===========
    // =======================

    event_loop.run(move |event, _, control_flow| {
        let frame_time = std::time::Instant::now();

        // event handling

        match event {
            event::Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
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

        // draw

        let view = math::view_matrix(
            Vector3::new(0.0, 1.0, -1.0),
            figure3.mesh.transform.get_position()
                - Vector3::new(0.0, 1.0, -1.0),
            Vector3::new(0.0, 1.0, 0.0));

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        let mut renderer = Renderer::new(
            &display,
            [0.0, 0.0, 0.0],
            [1.4, 0.4, -0.7f32],
            view,
            math::perspective(1920, 1080, 3.141592 / 3.0, 1024.0, 0.1),
            params,
        );

        renderer.clear(0.6, 0.8, 0.2, 1.0);
        renderer.draw(&board.mesh);
        renderer.draw(&figure.mesh);
        renderer.draw(&figure2.mesh);
        renderer.draw(&figure3.mesh);
        renderer.show();

        // update

        // x * x1 + z * z1 = 0
        // x * sign(x) + z * z1 = 0
        // x * x1 = -(z * z1)

        let delta = speed * frame_time.elapsed().as_secs_f32();
        let delta2 = speed2 * frame_time.elapsed().as_secs_f32();

        // if figure.mesh.transform.get_position().z / figure.mesh.transform.get_scale().z < 1.0 {
        //     figure.mesh.transform.translate(Vector3::new(0.0, 0.0, delta));
        // }
        figure2.mesh.transform.set_position(Vector3::new(angle2.sin(), 0.0, angle2.cos()) * figure2.mesh.transform.get_scale().z * 2.0);
        figure3.mesh.transform.set_position(Vector3::new(angle.sin(), 0.0, angle.cos()) * figure3.mesh.transform.get_scale().z * 3.0);
        angle += delta;
        angle2 += delta2;
    });
}