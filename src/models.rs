use nannou::prelude::*;
use nannou::rand::thread_rng;
use rand_distr::{Distribution, Normal};

#[derive(Debug, Default, Clone)]
pub struct Line {
    pub thickness: f32,
    pub points: Vec<Point2>,
}

impl Line {
    pub fn new(start: Point2, end: Point2, thickness: f32, segments: u32) -> Self {
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

        line = line.smooth(6).thicken(0);

        line
    }

    pub fn smooth(self, iterations: u8) -> Self {
        let mut points = self.points.clone();
        points = self.smooth_points(points, iterations);
        points = self.avg_points(points);

        Self {
            thickness: self.thickness,
            points,
        }
    }

    fn smooth_points(&self, points: Vec<Point2>, iterations: u8) -> Vec<Point2> {
        if iterations == 0 {
            return points;
        }

        let len = points.len();
        let mut smooth = vec![];

        for (i, point) in points.iter().enumerate() {
            let p1 = Point2::new(
                0.75 * point.x + 0.25 * points[(i + 1) % len].x,
                0.75 * point.y + 0.25 * points[(i + 1) % len].y,
            );

            let p2 = Point2::new(
                0.25 * point.x + 0.75 * points[(i + 1) % len].x,
                0.25 * point.y + 0.75 * points[(i + 1) % len].y,
            );

            let mut pta = vec![p1, p2];

            smooth.append(&mut pta);
        }

        return self.smooth_points(smooth, iterations - 1);
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

    pub fn thicken(mut self, amt: u8) -> Self {
        if amt == 0 {
            return self;
        }

        let mut points = vec![];

        let mut rng = thread_rng();

        self.points.iter().for_each(|point| {
            for _ in 0..amt {
                let norm = Normal::new(0.0f32, 2.0f32).unwrap();
    
                let nx = norm.sample(&mut rng);
                let ny = norm.sample(&mut rng);
    
                let mut new = Point2::new(nx, ny);
                new = new + *point;
                points.push(new);
            }

        });

        points.append(&mut self.points);

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

            let line = Line::new(start, end, 1.0, 5);

            lines.push(line);
        }
        Model { window: w, lines }
    }
}
