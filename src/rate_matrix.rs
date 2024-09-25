use crate::Tree;

#[derive(Debug)]
pub struct RateParam(pub f64, pub f64, pub f64, pub f64, pub f64, pub f64, pub Vec<f64>);

impl Default for RateParam {
    fn default() -> Self {
        RateParam(4.0 / 3.0, 4.0 / 3.0, 4.0 / 3.0, 4.0 / 3.0, 4.0 / 3.0, 4.0 / 3.0, vec![0.25, 0.25, 0.25, 0.25])
    }
}

impl Tree {

    pub fn update_rate_param(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, pv: Vec<f64>) {
        self.rate_param = RateParam(a, b, c, d, e, f, pv);
        // self.update_rate_matrix_GTR();
    }

    pub fn update_rate_matrix_GTR(&mut self) {
        let RateParam(a, b, c, d, e, f, pv) = &self.rate_param;
        // pv = pivec defined as (piA, piC, piG, piT)
        let mut q = na::Matrix4::new(
            -(a * pv[1] + b * pv[2] + c * pv[3]),
            a * pv[1],
            b * pv[2],
            c * pv[3],
            a * pv[0],
            -(a * pv[0] + d * pv[2] + e * pv[3]),
            d * pv[2],
            e * pv[3],
            b * pv[0],
            d * pv[1],
            -(b * pv[0] + d * pv[1] + f * pv[3]),
            f * pv[3],
            c * pv[0],
            e * pv[1],
            f * pv[2],
            -(c * pv[0] + e * pv[1] + f * pv[2]));

        self.rate_matrix = q;
    }
}