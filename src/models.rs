use fastcurve_3d::fast_curve_2d;
use nannou::prelude::*;
use nannou::rand::{thread_rng, Rng, distributions::Uniform};
use rand_distr::{Normal, Distribution};


#[derive(Debug, Default, Clone)]
pub struct Line {
    pub thickness: f32,
    pub points: Vec<Point2>,
}

impl Line {
    pub fn new(start: Point2, end: Point2, thickness: f32, segments: u32, wobbliness: f32) -> Self {
        let mut points = vec![start];

        let dx = (end.x - start.x) / segments as f32;
        let dy = (end.y - start.y) / segments as f32;

        for i in 0..segments {
            let mut new_x = start.x + (i as f32) * dx;
            new_x = new_x + random_range(-5.0, 5.0);

            let mut new_y = start.y + (i as f32) * dy;
            new_y = new_y + random_range(-5.0, 5.0);

            let new_point = Point2::new(new_x, new_y);
            points.push(new_point);
        }

        points.push(end);

        let mut line = Self { thickness, points };

        line = line.smooth(3).thicken(100);

        line
    }

    pub fn smooth(self, iterations: u8) -> Self {
        if iterations == 0 {
            return self;
        }

        let xs: Vec<f64> = self.points.iter().map(|p| p.x as f64).collect();
        let ys: Vec<f64> = self.points.iter().map(|p| p.y as f64).collect();

        let (xn, yn) = fast_curve_2d(&xs, &ys, iterations);

        let mut points = vec![];

        for i in 0..xn.len() {
            let mut p: Point2;

            if i == 0 || i + 1 >= xn.len() {
                p = Point2::new(xn[i] as f32, yn[i] as f32);
                points.push(p);
                continue;
            }

            p = Point2::new(xn[i] as f32, yn[i] as f32);
            let prev_pt = Point2::new(xn[i - 1] as f32, yn[i - 1] as f32);
            let next_pt = Point2::new(xn[i + 1] as f32, yn[i + 1] as f32);

            p = (p + prev_pt + next_pt) / 3.0;

            points.push(p);
        }

        Self {
            thickness: self.thickness,
            points,
        }
    }

    pub fn thicken(self, amt: u8) -> Self {
        let mut points = vec![];

        let mut rng = thread_rng();

        self.points.iter().for_each(|point| {
            points.push(*point);

            for _ in 0..amt {
                let rx = Normal::new(2.0f32, 2.0f32).unwrap();
                let ry = Normal::new(2.0f32, 2.0f32).unwrap();

                let mut nx = rx.sample(&mut rng);
                let mut ny = ry.sample(&mut rng);
                

    
                nx = nx + point.x;
                ny = ny + point.y;
    
                points.push(Point2::new(nx, ny));
            }
        });

        Self {
            thickness: self.thickness,
            points,
        }
    }
}

pub struct Model {
    pub window: WindowId,
    pub lines: Vec<Line>,
}

const MAX_LINES: u8 = 8;

impl Model {
    pub fn new(w: WindowId) -> Model {
        let mut lines = vec![];

        for i in 0..MAX_LINES {
            let mut sx = -100.0;
            let mut sy = ((i as f32) - (MAX_LINES as f32) / 2.0) * 50.0;

            sx += random_range(-5.0, 5.0);
            sy += random_range(-5.0, 5.0);

            let mut ex = 100.0;
            let mut ey = ((i as f32) - (MAX_LINES as f32) / 2.0) * 50.0;

            ex += random_range(-5.0, 5.0);
            ey += random_range(-5.0, 5.0);

            let start = Point2::new(sx, sy);
            let end = Point2::new(ex, ey);

            let line = Line::new(start, end, 1.0, 10, 1.0);

            lines.push(line);
        }
        Model { window: w, lines }
    }
}
