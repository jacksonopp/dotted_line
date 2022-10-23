use std::vec;

use fastcurve_3d::fast_curve_2d;
use nannou::prelude::*;
use nannou::rand::thread_rng;
use rand_distr::{Distribution, Normal};

#[derive(Debug, Default, Clone)]
pub struct Line {
    pub thickness: f32,
    pub points: Vec<Point2>,
    pub orig_points: Vec<Point2>,
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

        let mut line = Self {
            thickness,
            orig_points: points.clone(),
            points,
        };

        line = line.smooth(6).thicken(10);

        line
    }

    pub fn smooth(self, iterations: u8) -> Self {
        let mut points = self.points.clone();
        points = self.chaikin_smooth_points(points, iterations);
        // points = self.fast_smooth(points, iterations);
        points = self.avg_points(points);

        Self {
            thickness: self.thickness,
            points,
            orig_points: self.points,
        }
    }

    fn fast_smooth(&self, points: Vec<Point2>, iterations: u8) -> Vec<Point2> {
        let xs: Vec<f64> = points.iter().map(|p| p.x as f64).collect();
        let ys: Vec<f64> = points.iter().map(|p| p.y as f64).collect();

        let (xn, yn) = fast_curve_2d(&xs, &ys, iterations);

        let mut points = vec![];

        for i in 0..xn.len() {
            let point = Point2::new(xn[i] as f32, yn[i] as f32);
            points.push(point);
        }

        points
    }

    fn chaikin_smooth_points(&self, points: Vec<Point2>, iterations: u8) -> Vec<Point2> {
        if iterations == 0 {
            return points;
        }

        let mut p = self.chaikin_step(points);
        for _ in 0..iterations - 1 {
            p = self.chaikin_step(p);
        }
        p
    }

    fn chaikin_step(&self, points: Vec<Point2>) -> Vec<Point2> {
        let mut p = vec![];

        for i in 1..points.len() - 1 {
            let p1 = Point2::new(
                0.25 * points[i - 1].x + 0.75 * points[i].x,
                0.25 * points[i - 1].y + 0.75 * points[i].y,
            );
            let p2 = Point2::new(
                0.75 * points[i - 1].x + 0.25 * points[i].x,
                0.75 * points[i - 1].y + 0.25 * points[i].y,
            );

            p.push(p1);
            p.push(p2);
        }

        p
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

    pub fn thicken(self, _amt: u8) -> Self {
        let mut p = vec![];
        let mut rng = thread_rng();
        let normal = Normal::new(0.0f32, 0.5f32).unwrap();

        for point in self.points.iter() {
            let x = normal.sample(&mut rng);
            let y = normal.sample(&mut rng);

            let mut new_point = Point2::new(x, y);
            new_point = *point + new_point;

            p.push(new_point);
        }

        p = self.chaikin_smooth_points(p, 3);

        Self {
            thickness: self.thickness,
            points: p,
            orig_points: self.orig_points,
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
            let mut sx = -500.0;
            let mut sy = ((i as f32) - (MAX_LINES as f32) / 2.0) * 50.0;

            sx += random_range(-5.0, 5.0);
            sy += random_range(-5.0, 5.0);

            let mut ex = 500.0;
            let mut ey = ((i as f32) - (MAX_LINES as f32) / 2.0) * 50.0;

            ex += random_range(-5.0, 5.0);
            ey += random_range(-5.0, 5.0);

            let start = Point2::new(sx, sy);
            let end = Point2::new(ex, ey);

            let line = Line::new(start, end, 1.0, 10);

            lines.push(line);
        }
        Model { window: w, lines }
    }
}
