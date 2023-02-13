use core::ffi::CStr;
use pgx::prelude::*;
use pgx::{InOutFuncs, StringInfo};
use serde::{Deserialize, Serialize};

fn compute_digit(mut pis: i64) -> i64 {
    let mut digits: [i64; 10] = [0; 10];

    for i in (0..=9).rev() {
        digits[i] = pis % 10;
        pis = pis / 10
    }

    let mut ret = (3 * digits[0]) + (2 * digits[1]) +
            (9 * digits[2]) + (8 * digits[3]) +
            (7 * digits[4]) + (6 * digits[5]) +
            (5 * digits[6]) + (4 * digits[7]) +
            (3 * digits[8]) + (2 * digits[9]);
    ret = 11 - (ret % 11);
    if ret > 9 {
        ret = 0;
    }
    ret
}

#[derive(Eq, PartialEq, Ord, Hash, PartialOrd)]
#[derive(Copy, Clone, PostgresType, Serialize, Deserialize)]
#[derive(PostgresEq)]
#[derive(PostgresOrd)]
#[derive(PostgresHash)]
#[inoutfuncs]
pub struct Pis(i64);

impl InOutFuncs for Pis {
    fn input(_input: &CStr) -> Self {
        let s = _input.to_str().unwrap();
        let n = s.parse::<i64>().unwrap();
 
        if n < 10 || n > 99999999999 {
            pgx::error!("PIS must be between 10 and 99999999999");
        }

        let dv = n % 10;
        let digit = compute_digit(n / 10);

        if dv != digit {
            pgx::error!("invalid check digit for PIS");
        }

        Pis(n)
    }

    fn output(&self, buffer: &mut StringInfo) {
        let mut group: [i64; 4] = [0; 4];

        // 999.99999.99-9
        group[0] = (self.0 / 100000000) % 1000;
        group[1] = (self.0 / 1000) % 100000;
        group[2] = (self.0 / 10) % 100;
        group[3] = self.0 % 10;

        buffer.push_str(&format!("{:03}.{:05}.{:02}-{:1}",
                group[0], group[1], group[2], group[3]));
    }
}
