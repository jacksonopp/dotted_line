use fastcurve_3d::fast_curve_2d;
use nannou::prelude::*;
use nannou::rand::{distributions::Uniform, thread_rng, Rng};
use rand_distr::{Distribution, Normal};

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

        line = line.smooth(3).thicken(5);

        line
    }


    pub fn smooth(self, iterations: u8) -> Self {
        if iterations == 0 {
            return self;
        };

        let len = self.points.len();

        let mut smooth = vec![];

        for (i, point) in self.points.iter().enumerate() {
            let new_x1 = 0.75*point.x + 0.25*self.points[(i+1)%len].x;
            let new_y1 = 0.75*point.y + 0.25*self.points[(i+1)%len].y;
            let pt1 = Point2::new(new_x1, new_y1);
            
            let new_x2 = 0.25*point.x + 0.75*self.points[(i+1)%len].x;
            let new_y2 = 0.25*point.y + 0.75*self.points[(i+1)%len].y;
            let pt2 = Point2::new(new_x2, new_y2);

            smooth.push(pt1);
            smooth.push(pt2);
        }

        let points = self.avg_points(smooth);

        let new_line = Self { thickness: self.thickness, points };

        if iterations == 1 {
            return new_line
        } else {
            return new_line.smooth(iterations - 1);
        }
    }

    fn smooth_points(&self, points: Vec<Point2>, iterations: u8) -> Vec<Point2> {
        if iterations == 0 {
            return points;
        }

        let len = points.len();
        let mut smooth = vec![];

        for (i, point) in points.iter().enumerate() {
            let p = Point2::new(
                0.75*point.x + 0.25*points[(i+1)%len].x,
                0.75*point.y + 0.25*points[(i+1)%len].y,
            );
            smooth.push(p);
            
            let p = Point2::new(
                0.25*point.x + 0.75*points[(i+1)%len].x,
                0.25*point.y + 0.75*points[(i+1)%len].y,
            );
            smooth.push(p);
        }

        if iterations == 1 {
            return smooth;
        } else {
            return self.smooth_points(smooth, iterations - 1)
        }
    }

    fn avg_points(&self, points: Vec<Point2>) -> Vec<Point2> {
        let mut p = vec![];
        for (i, point) in points.iter().enumerate() {
            if i == 0 || i + 1 >= points.len() {
                p.push(*point);
                continue;
            }

            let prev = points[i - 1];
            let next = points[i + 1];
            let new = (prev + next + *point) / 3.0;
            p.push(new)
        }

        p
    }

    pub fn thicken(self, amt: u8) -> Self {
        let mut points = vec![];

        let mut rng = thread_rng();

        self.points.iter().for_each(|point| {
            points.push(*point);

            if amt > 0 {
                for _ in 0..amt {
                    let rx = Normal::new(0.0f32, 2.0f32).unwrap();
                    let ry = Normal::new(0.0f32, 2.0f32).unwrap();

                    let mut nx = rx.sample(&mut rng);
                    let mut ny = ry.sample(&mut rng);

                    nx = nx + point.x;
                    ny = ny + point.y;

                    points.push(Point2::new(nx, ny));
                }
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

            let line = Line::new(start, end, 1.0, 5, 1.0);

            lines.push(line);
        }
        Model { window: w, lines }
    }
}
