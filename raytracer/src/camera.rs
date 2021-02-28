use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread,
    time::Instant,
};

use crate::{
    canvas::Canvas,
    color::Color,
    matrix::Matrix,
    point::Point,
    ray::Ray,
    world::{World, MAX_RECURSION_DEPTH},
};

#[derive(Debug)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    _field_of_view: f64,
    transform: Matrix,
    transform_inverse: Matrix,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
    pub render_opts: RenderOpts,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = half_width * 2.0 / hsize as f64;

        Self {
            hsize,
            vsize,
            _field_of_view: field_of_view,
            transform: Matrix::identity(4, 4),
            transform_inverse: Matrix::identity(4, 4),
            pixel_size,
            half_width,
            half_height,
            render_opts: RenderOpts::default(),
        }
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = &self.transform_inverse * Point::new(world_x, world_y, -1.0);
        let origin = &self.transform_inverse * Point::origin();
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn rays_for_pixel(&self, px: usize, py: usize) -> Vec<Ray> {
        let mut rays = vec![];
        let offsets = Self::get_offsets(&self.render_opts.aa_samples);

        for offset in offsets.iter() {
            let xoffset = (px as f64 + offset.0) * self.pixel_size;
            let yoffset = (py as f64 + offset.1) * self.pixel_size;

            let world_x = self.half_width - xoffset;
            let world_y = self.half_height - yoffset;

            let pixel = &self.transform_inverse * Point::new(world_x, world_y, -1.0);
            let origin = &self.transform_inverse * Point::origin();
            let direction = (pixel - origin).normalize();

            rays.push(Ray::new(origin, direction));
        }

        rays
    }

    fn get_offsets(samples: &AASamples) -> Vec<(f64, f64)> {
        match samples {
            AASamples::X1 => vec![(0.5, 0.5)],
            AASamples::X2 => vec![(0.25, 0.5), (0.75, 0.5)],
            AASamples::X4 => vec![(0.25, 0.25), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)],
            AASamples::X8 => vec![
                (0.25, 0.25),
                (0.5, 0.25),
                (0.75, 0.25),
                (0.25, 0.5),
                (0.75, 0.5),
                (0.25, 0.75),
                (0.5, 0.75),
                (0.75, 0.75),
            ],
            AASamples::X16 => vec![
                (0.125, 0.125),
                (0.375, 0.125),
                (0.625, 0.125),
                (0.875, 0.125),
                (0.125, 0.375),
                (0.375, 0.375),
                (0.625, 0.375),
                (0.875, 0.375),
                (0.125, 0.625),
                (0.375, 0.625),
                (0.625, 0.625),
                (0.875, 0.625),
                (0.125, 0.875),
                (0.375, 0.875),
                (0.625, 0.875),
                (0.875, 0.875),
            ],
        }
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
        self.transform_inverse = self.transform.inverse();
    }

    pub fn render(&mut self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            if y % 10 == 0 {
                println!("rendering row {}/{}", y, self.vsize);
            }
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray, MAX_RECURSION_DEPTH);
                image.set_pixel(x, y, color);
            }
        }

        image
    }

    pub fn render_multithreaded(this: Arc<Self>, world: Arc<World>) -> Canvas {
        let mut image = Canvas::new(this.hsize, this.vsize);

        let mut handles = vec![];
        let (tx, rx): (Sender<RenderThreadResult>, Receiver<RenderThreadResult>) = mpsc::channel();
        let rows = this.vsize;
        let num_threads = this.render_opts.num_threads;
        let rows_per_thread = rows / num_threads;

        println!(
            "running with {} threads: assigning {} rows per thread",
            num_threads, rows_per_thread
        );
        let start_time = Instant::now();
        for i in 0..num_threads {
            let camera_ref = this.clone();
            let world_ref = world.clone();
            let tx_ref = tx.clone();
            let handle = thread::spawn(move || {
                let (start, mut end) = (i * rows_per_thread, i * rows_per_thread + rows_per_thread);
                if i == num_threads - 1 {
                    end = rows;
                }
                let mut result = RenderThreadResult {
                    start,
                    end,
                    colors: vec![],
                };
                for y in start..end {
                    for x in 0..camera_ref.hsize {
                        let rays = camera_ref.rays_for_pixel(x, y);
                        let mut colors = vec![];
                        for ray in rays.iter() {
                            let color = world_ref.color_at(&ray, MAX_RECURSION_DEPTH);
                            colors.push(color);
                        }
                        let color = Color::average(&colors);
                        result.colors.push(color);
                    }
                }
                tx_ref.send(result).unwrap();
            });
            handles.push(handle);
        }

        for _ in 0..num_threads {
            let res = rx
                .recv()
                .expect("failed to receive render result from thread");
            println!("received colors array from thread");
            let mut i = 0;
            for y in res.start..res.end {
                for x in 0..this.hsize {
                    image.set_pixel(x, y, res.colors[i]);
                    i += 1;
                }
            }
        }

        let elapsed_time = start_time.elapsed().as_millis();
        println!("rendered in {} ms", elapsed_time);

        for handle in handles {
            handle.join().expect("could not join thread handle");
        }
        println!("all render threads done!");
        image
    }
}

#[derive(Debug)]
pub struct RenderOpts {
    num_threads: usize,
    aa_samples: AASamples,
}

#[derive(Debug)]
pub enum AASamples {
    X1,
    X2,
    X4,
    X8,
    X16,
}

impl Default for RenderOpts {
    fn default() -> Self {
        Self {
            num_threads: 1,
            aa_samples: AASamples::X1,
        }
    }
}

impl RenderOpts {
    pub fn num_threads(&mut self, n: usize) {
        assert!(n > 0);
        self.num_threads = n;
    }

    pub fn aa_samples(&mut self, samples: AASamples) {
        self.aa_samples = samples;
    }
}

struct RenderThreadResult {
    start: usize,
    end: usize,
    colors: Vec<Color>,
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{
        color::Color,
        equal,
        transform::{rotation_y, translation, view_transform},
        vector::Vector,
        world::World,
    };

    use super::*;

    #[test]
    fn create_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;
        let c = Camera::new(hsize, vsize, field_of_view);
        assert_eq!(c.hsize, hsize);
        assert_eq!(c.vsize, vsize);
        assert!(equal(c._field_of_view, field_of_view));
        assert_eq!(c.transform, Matrix::identity(4, 4));
    }

    #[test]
    fn pixel_size_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert!(equal(c.pixel_size, 0.01));
    }

    #[test]
    fn pixel_size_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert!(equal(c.pixel_size, 0.01));
    }

    #[test]
    fn construct_ray_canvas_center() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin(), Point::origin());
        assert_eq!(r.direction(), Vector::new(0, 0, -1));
    }

    #[test]
    fn construct_ray_canvas_corner() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin(), Point::origin());
        assert_eq!(r.direction(), Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn construct_ray_transformed_camera() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.set_transform(&rotation_y(PI / 4.0) * &translation(0, -2, 5));
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin(), Point::new(0, 2, -5));
        assert_eq!(
            r.direction(),
            Vector::new(2.0f64.sqrt() / 2.0, 0.0, -(2.0f64.sqrt() / 2.0))
        );
    }

    #[test]
    fn render_world_with_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = Point::new(0, 0, -5);
        let to = Point::origin();
        let up = Vector::new(0, 1, 0);
        c.set_transform(view_transform(from, to, up));
        let image = c.render(&w);
        assert_eq!(image.get_pixel(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
