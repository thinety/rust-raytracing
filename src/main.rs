mod entity;
mod image;
mod material;
mod math;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use entity::{Entity, Sphere};
use image::{
    camera::{Camera, CameraControls},
    pixels::output,
    Image,
};
use material::{BlackBody, BlackBodyNormal, Dielectric, Lambertian, Material, Metal};
use math::{Color, Point, Vector};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;

    let event_loop = EventLoop::new();

    let mut window_grabbed = false;
    let window = {
        let scale_factor = 1;
        let window_width = scale_factor * image_width;
        let window_height = scale_factor * image_height;

        WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(window_width, window_height))
            .with_resizable(false)
            .with_title("Raytracer")
            .build(&event_loop)
            .expect("Failed to create window")
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

        Pixels::new(image_width, image_height, surface_texture)
            .expect("Failed to create pixel buffer")
    };

    let mut image = {
        let image_width = image_width as usize;
        let image_height = image_height as usize;

        Image::new(image_width, image_height)
    };

    let mut camera_controls = CameraControls {
        moving_forward: false,
        moving_backward: false,
        moving_left: false,
        moving_right: false,
        moving_up: false,
        moving_down: false,
        mouse_delta: (0.0, 0.0),
    };
    let mut camera = {
        let v_up = Vector::new(0.0, 0.0, 1.0);
        let v_fov = 90.0;

        Camera::new(v_up, v_fov, aspect_ratio)
    };

    let world = vec![
        Entity::from(Sphere {
            center: Point::new(0.0, 0.0, 0.0),
            radius: 0.5,
            material: Material::from(Metal {
                albedo: Color::new(0.8, 0.8, 0.8),
                fuzz: 0.0,
            }),
        }),
        Entity::from(Sphere {
            center: Point::new(0.0, 0.0, -2.0),
            radius: 0.5,
            material: Material::from(BlackBody {
                color: Color::new(0.0, 0.0, 0.0),
            }),
        }),
        Entity::from(Sphere {
            center: Point::new(0.0, 0.0, 2.0),
            radius: 0.5,
            material: Material::from(BlackBody {
                color: Color::new(1.0, 1.0, 1.0),
            }),
        }),
        Entity::from(Sphere {
            center: Point::new(2.0, 0.0, 0.0),
            radius: 0.5,
            material: Material::from(BlackBodyNormal {}),
        }),
        Entity::from(Sphere {
            center: Point::new(0.0, 2.0, 0.0),
            radius: 0.5,
            material: Material::from(Lambertian {
                albedo: Color::new(1.0, 0.0, 0.0),
            }),
        }),
        Entity::from(Sphere {
            center: Point::new(-2.0, 0.0, 0.0),
            radius: 0.5,
            material: Material::from(Lambertian {
                albedo: Color::new(0.0, 1.0, 0.0),
            }),
        }),
        Entity::from(Sphere {
            center: Point::new(0.0, -2.0, 0.0),
            radius: 0.5,
            material: Material::from(Lambertian {
                albedo: Color::new(0.0, 0.0, 1.0),
            }),
        }),
        Entity::from(Sphere {
            center: Point::new(5.0, 5.0, 5.0),
            radius: 2.0,
            material: Material::from(Dielectric {
                albedo: Color::new(1.0, 1.0, 1.0),
                refraction_index: 1.5,
            }),
        }),
    ];

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::MainEventsCleared => {
                camera.update(&mut camera_controls);

                image.render_threaded(&camera, &world, 50, 1, 8);

                output(&image, pixels.get_frame());

                pixels.render().expect("Failed to render");
            }
            Event::DeviceEvent { event, .. } => {
                match event {
                    DeviceEvent::Key(input) => {
                        if let Some(key_code) = input.virtual_keycode {
                            match (input.state, key_code) {
                                (ElementState::Pressed, VirtualKeyCode::W) if window_grabbed => {
                                    camera_controls.moving_forward = true;
                                }
                                (ElementState::Released, VirtualKeyCode::W) if window_grabbed => {
                                    camera_controls.moving_forward = false;
                                }
                                (ElementState::Pressed, VirtualKeyCode::S) if window_grabbed => {
                                    camera_controls.moving_backward = true;
                                }
                                (ElementState::Released, VirtualKeyCode::S) if window_grabbed => {
                                    camera_controls.moving_backward = false;
                                }
                                (ElementState::Pressed, VirtualKeyCode::A) if window_grabbed => {
                                    camera_controls.moving_left = true;
                                }
                                (ElementState::Released, VirtualKeyCode::A) if window_grabbed => {
                                    camera_controls.moving_left = false;
                                }
                                (ElementState::Pressed, VirtualKeyCode::D) if window_grabbed => {
                                    camera_controls.moving_right = true;
                                }
                                (ElementState::Released, VirtualKeyCode::D) if window_grabbed => {
                                    camera_controls.moving_right = false;
                                }
                                (ElementState::Pressed, VirtualKeyCode::Space)
                                    if window_grabbed =>
                                {
                                    camera_controls.moving_up = true;
                                }
                                (ElementState::Released, VirtualKeyCode::Space)
                                    if window_grabbed =>
                                {
                                    camera_controls.moving_up = false;
                                }
                                (ElementState::Pressed, VirtualKeyCode::LShift)
                                    if window_grabbed =>
                                {
                                    camera_controls.moving_down = true;
                                }
                                (ElementState::Released, VirtualKeyCode::LShift)
                                    if window_grabbed =>
                                {
                                    camera_controls.moving_down = false;
                                }
                                (ElementState::Pressed, VirtualKeyCode::Escape) => {
                                    if window_grabbed {
                                        window.set_cursor_grab(false).unwrap();
                                        window.set_cursor_visible(true);
                                        window_grabbed = false;
                                    } else {
                                        window.set_cursor_grab(true).unwrap();
                                        window.set_cursor_visible(false);
                                        window_grabbed = true;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    DeviceEvent::MouseMotion { delta } if window_grabbed => {
                        camera_controls.mouse_delta.0 += delta.0;
                        camera_controls.mouse_delta.1 += delta.1;
                    }
                    _ => {}
                }
            }
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });
}
