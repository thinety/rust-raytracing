use std::f64::consts;

use crate::math::{Point, Ray, Vector};

pub struct Camera {
    v_up: Vector,
    viewport_height: f64,
    viewport_width: f64,
    look_from: Point,
    theta: f64,
    phi: f64,
}

pub struct CameraControls {
    pub moving_forward: bool,
    pub moving_backward: bool,
    pub moving_left: bool,
    pub moving_right: bool,
    pub moving_up: bool,
    pub moving_down: bool,
    pub mouse_delta: (f64, f64),
}

impl Camera {
    pub fn new(v_up: Vector, v_fov: f64, aspect_ratio: f64) -> Self {
        let v_fov = v_fov.to_radians();
        let h = (v_fov / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        Self {
            v_up,
            viewport_height,
            viewport_width,
            look_from: Point::new(0.0, 0.0, 0.0),
            theta: consts::FRAC_PI_2,
            phi: consts::FRAC_PI_2,
        }
    }

    fn get_base(&self) -> (Vector, Vector, Vector) {
        let cos_phi = self.phi.cos();
        let sin_phi = self.phi.sin();
        let cos_theta = self.theta.cos();
        let sin_theta = self.theta.sin();

        let w = Vector::new(sin_phi * cos_theta, sin_phi * sin_theta, cos_phi);
        let u = Vector::cross(&w, &self.v_up).unit();
        let v = Vector::cross(&w, &u).unit();

        // u, v and w together form an orthonormal base
        (u, v, w)
    }

    fn update_look_from(&mut self, controls: &CameraControls) {
        let (u, v, w) = self.get_base();

        let mut delta = Vector::new(0.0, 0.0, 0.0);

        if controls.moving_forward && !controls.moving_backward {
            delta += w;
        } else if !controls.moving_forward && controls.moving_backward {
            delta -= w;
        }

        if controls.moving_right && !controls.moving_left {
            delta += u;
        } else if !controls.moving_right && controls.moving_left {
            delta -= u;
        }

        if controls.moving_up && !controls.moving_down {
            delta -= v;
        } else if !controls.moving_up && controls.moving_down {
            delta += v;
        }

        if delta.length_squared() != 0.0 {
            self.look_from += 0.1 * delta.unit();
        }
    }

    fn update_look_at(&mut self, controls: &CameraControls) {
        self.theta -= 0.001 * controls.mouse_delta.0;
        self.phi += 0.001 * controls.mouse_delta.1;

        let two_pi = 2.0 * consts::PI;
        if self.theta >= two_pi {
            self.theta -= two_pi;
        } else if self.theta < 0.0 {
            self.theta += two_pi;
        }

        let pi = consts::PI;
        if self.phi >= pi {
            self.phi = pi - 0.0001;
        } else if self.phi <= 0.0 {
            self.phi = 0.0001;
        }
    }

    pub fn update(&mut self, controls: &mut CameraControls) {
        self.update_look_from(controls);
        self.update_look_at(controls);
        controls.mouse_delta = (0.0, 0.0);
    }

    pub fn get_ray(&self, i: f64, j: f64) -> Ray {
        let (u, v, w) = self.get_base();

        let direction =
            (w + (i - 0.5) * self.viewport_height * v + (j - 0.5) * self.viewport_width * u).unit();

        Ray {
            origin: self.look_from,
            direction,
        }
    }
}
