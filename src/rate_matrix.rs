use crate::Tree;

pub trait RateMatrix: Copy {
    fn update_matrix(&mut self);

    fn update_params(&mut self, params: Vec<f64>);

    fn get_matrix(&self) -> na::Matrix4<f64>;

    fn get_params(&self) -> Vec<f64>;

}

#[derive(Debug, Clone, Copy)]
pub struct GTR {
    matrix: na::Matrix4<f64>,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
    p0: f64,
    p1: f64,
    p2: f64,
    p3: f64,
}

impl RateMatrix for GTR {

    fn get_params(&self) -> Vec<f64> {
        vec![self.a, self.b, self.c, self.d, self.e, self.f, self.p0, self.p1, self.p2, self.p3]
    }

    fn update_params(&mut self, params: Vec<f64>) {
        self.a = params[0];
        self.b = params[1];
        self.c = params[2];
        self.d = params[3];
        self.e = params[4];
        self.f = params[5];
        self.p0 = params[6];
        self.p1 = params[7];
        self.p2 = params[8];
        self.p3 = params[9];
        self.update_matrix();
    }

    fn get_matrix(&self) -> na::Matrix4<f64> {
        self.matrix
    }

    fn update_matrix(&mut self) {
        
        self.matrix = na::Matrix4::new(
                -(self.a * self.p1 + self.b * self.p2 + self.c * self.p3),
                self.a * self.p1,
                self.b * self.p2,
                self.c * self.p3,
                self.a * self.p0,
                -(self.a * self.p0 + self.d * self.p2 + self.e * self.p3),
                self.d * self.p2,
                self.e * self.p3,
                self.b * self.p0,
                self.d * self.p1,
                -(self.b * self.p0 + self.d * self.p1 + self.f * self.p3),
                self.f * self.p3,
                self.c * self.p0,
                self.e * self.p1,
                self.f * self.p2,
                -(self.c * self.p0 + self.e * self.p1 + self.f * self.p2));
    }
}

impl Default for GTR {
    fn default() -> Self{
        let mut out: GTR = GTR { matrix: na::Matrix4::identity(),
            a: 4.0 / 3.0,
            b: 4.0 / 3.0,
            c: 4.0 / 3.0,
            d: 4.0 / 3.0,
            e: 4.0 / 3.0,
            f: 4.0 / 3.0,
            p0: 0.25,
            p1: 0.25,
            p2: 0.25,
            p3: 0.25,
        };
        out.update_matrix();
        out
    }
}

#[derive(Debug, Clone, Copy)]
pub struct JC69 {
    matrix: na::Matrix4<f64>,
    mu: f64,
}

impl RateMatrix for JC69 {
    fn get_matrix(&self) -> na::Matrix4<f64> {
        self.matrix
    }

    fn get_params(&self) -> Vec<f64> {
        vec![self.mu]
    }

    fn update_params(&mut self, params: Vec<f64>) {
        self.mu = params[0];
    }

    fn update_matrix(&mut self) {
        self.matrix = na::Matrix4::new(
            - (3.0 * self.mu) / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            - (3.0 * self.mu) / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            - (3.0 * self.mu) / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            - (3.0 * self.mu) / 4.0,
        );
    }
}

impl Default for JC69 {
    fn default() -> Self {
        let mut out = JC69{
            matrix: na::Matrix4::identity(),
            mu: 4.0 / 3.0,
        };
        out.update_matrix();
        out
    }
}