use crate::array_math;

pub struct DiagnosticArray {
    width: usize,
    log: Vec<u64>,
}

impl DiagnosticArray {
    pub fn new(log: Vec<u64>, width: usize) -> DiagnosticArray {
        if width < 1 || width > 64 {
            panic!{
                "Width of '{}' is not valid, must be between 1 and 64 inclusive",
                width,
            };
        }

        return DiagnosticArray{
            width,
            log,
        }

    }

    pub fn gamma(&self) -> i64{
        let mut gamma: i64 = 0;
        let l = self.log.len().try_into().unwrap();

        for i in 0..self.width {

            let filter: u64 = 1 << i;
            let count = array_math::count_true(
                &array_math::apply(
                    self.log.clone(),
                    |x| (x & filter) != 0,
                )
            );

            if 2*count < l {
                gamma += filter as i64;
            }

        }

        return gamma;

    }

    pub fn epsilon(&self) -> i64 {
        let epsilon: i64 = (1 << (self.width)) - 1;

        return epsilon ^ self.gamma();

    }

    pub fn oxygen_gen_rating(&self) -> i64 {

        let mut log = self.log.clone();

        for i in (0..self.width).rev() {

            let filter = 1 << i;

            let mut index = array_math::apply(log.clone(), |x| (x & filter) != 0);
            let count = array_math::count_true(&index);
            if (2*count as usize) < index.len() {
                array_math::not_inplace(&mut index);
            }

            log = array_math::args_where(&log, &index);

            if log.len() == 1 {
                return log[0] as i64;
            }

        }

        panic!("oh no");

    }

    pub fn c02_scrub_rating(&self) -> i64 {

        let mut log = self.log.clone();

        for i in (0..self.width).rev() {

            let filter = 1 << i;

            let mut index = array_math::apply(log.clone(), |x| (x & filter) != 0);
            let count = array_math::count_true(&index);
            if (2*count as usize) >= index.len() {
                array_math::not_inplace(&mut index);
            }

            log = array_math::args_where(&log, &index);

            if log.len() == 1 {
                return log[0] as i64;
            }

        }

        panic!("oh no");

    }

}