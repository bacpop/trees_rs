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

// #[derive(Debug)]
// pub struct RateParam(pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub Vec<f64>);

// impl Default for RateParam {
//     fn default() -> Self {
//         RateParam(4.0 / 3.0, 4.0 / 3.0, 4.0 / 3.0, 4.0 / 3.0, 4.0 / 3.0, 4.0 / 3.0, vec![0.25, 0.25, 0.25, 0.25])
//     }
// }

// impl Tree {

//     pub fn update_rate_param(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, pv: Vec<f64>) {
//         self.rate_param = RateParam(a, b, c, d, e, f, pv);
//         // self.update_rate_matrix_GTR();
//     }

//     pub fn update_rate_matrix_GTR(&mut self) {
//         let RateParam(a, b, c, d, e, f, pv) = &self.rate_param;
//         // pv = pivec defined as (piA, piC, piG, piT)
//         let mut q = na::Matrix4::new(
//             -(a * pv[1] + b * pv[2] + c * pv[3]),
//             a * pv[1],
//             b * pv[2],
//             c * pv[3],
//             a * pv[0],
//             -(a * pv[0] + d * pv[2] + e * pv[3]),
//             d * pv[2],
//             e * pv[3],
//             b * pv[0],
//             d * pv[1],
//             -(b * pv[0] + d * pv[1] + f * pv[3]),
//             f * pv[3],
//             c * pv[0],
//             e * pv[1],
//             f * pv[2],
//             -(c * pv[0] + e * pv[1] + f * pv[2]));

//         self.rate_matrix = q;
//     }
// }