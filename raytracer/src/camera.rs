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

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
        self.transform_inverse = self.transform.inverse();
    }

    pub fn render(&self, world: &World) -> Canvas {
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

    pub fn render_multithreaded(this: Arc<Self>, world: Arc<World>, num_threads: usize) -> Canvas {
        let mut image = Canvas::new(this.hsize, this.vsize);

        let mut handles = vec![];
        let (tx, rx): (Sender<RenderThreadResult>, Receiver<RenderThreadResult>) = mpsc::channel();
        let rows_per_thread = this.vsize / num_threads;
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
                let (start, end) = (i * rows_per_thread, i * rows_per_thread + rows_per_thread);
                let mut result = RenderThreadResult {
                    start,
                    end,
                    colors: vec![],
                };
                for y in start..end {
                    // if y % 10 == 0 {
                    //     println!("rendering row in thread {} {}/{}", i, y, camera_ref.vsize);
                    // }
                    for x in 0..camera_ref.hsize {
                        let ray = camera_ref.ray_for_pixel(x, y);
                        let color = world_ref.color_at(&ray, MAX_RECURSION_DEPTH);
                        result.colors.push(color);
                    }
                }
                tx_ref.send(result).unwrap();
            });
            handles.push(handle);
        }

        // let camera_ref = this.clone();
        // let world_ref = world.clone();
        // let (tx, rx): (Sender<RenderThreadResult>, Receiver<RenderThreadResult>) = mpsc::channel();
        // let tx_ref = tx.clone();
        // let handle1 = thread::spawn(move || {
        //     let (start, end) = (0, camera_ref.vsize / 2);
        //     let mut result = RenderThreadResult {
        //         start,
        //         end,
        //         colors: vec![],
        //     };
        //     for y in start..end {
        //         if y % 10 == 0 {
        //             println!("rendering row in thread 1 {}/{}", y, camera_ref.vsize);
        //         }
        //         for x in 0..camera_ref.hsize {
        //             let ray = camera_ref.ray_for_pixel(x, y);
        //             let color = world_ref.color_at(&ray, MAX_RECURSION_DEPTH);
        //             result.colors.push(color);
        //         }
        //     }
        //     tx_ref.send(result).unwrap();
        // });

        // let camera_ref2 = this.clone();
        // let handle2 = thread::spawn(move || {
        //     let (start, end) = (camera_ref2.vsize / 2, camera_ref2.vsize);
        //     let mut result = RenderThreadResult {
        //         start,
        //         end,
        //         colors: vec![],
        //     };
        //     for y in start..end {
        //         if y % 10 == 0 {
        //             println!("rendering row in thread 2 {}/{}", y, camera_ref2.vsize);
        //         }

        //         for x in 0..camera_ref2.hsize {
        //             let ray = camera_ref2.ray_for_pixel(x, y);
        //             let color = world.color_at(&ray, MAX_RECURSION_DEPTH);
        //             result.colors.push(color);
        //         }
        //     }
        //     tx.send(result).unwrap();
        // });

        for _ in 0..num_threads {
            let res = rx.recv().unwrap();
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
