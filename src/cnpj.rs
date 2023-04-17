use core::ffi::CStr;
use pgrx::prelude::*;
use pgrx::{InOutFuncs, StringInfo};
use serde::{Deserialize, Serialize};

fn compute_digit(mut cnpj: i64) -> i64 {
    let mut digits: [i64; 12] = [0; 12];

    for i in (0..=11).rev() {
        digits[i] = cnpj % 10;
        cnpj = cnpj / 10
    }

    let mut d1 = (5 * digits[0])
        + (4 * digits[1])
        + (3 * digits[2])
        + (2 * digits[3])
        + (9 * digits[4])
        + (8 * digits[5])
        + (7 * digits[6])
        + (6 * digits[7])
        + (5 * digits[8])
        + (4 * digits[9])
        + (3 * digits[10])
        + (2 * digits[11]);
    d1 = 11 - (d1 % 11);
    if d1 > 9 {
        d1 = 0;
    }
    let mut d2 = (6 * digits[0])
        + (5 * digits[1])
        + (4 * digits[2])
        + (3 * digits[3])
        + (2 * digits[4])
        + (9 * digits[5])
        + (8 * digits[6])
        + (7 * digits[7])
        + (6 * digits[8])
        + (5 * digits[9])
        + (4 * digits[10])
        + (3 * digits[11])
        + (2 * d1);
    d2 = 11 - (d2 % 11);
    if d2 > 9 {
        d2 = 0;
    }
    (d1 * 10) + d2
}

#[derive(
    Eq,
    PartialEq,
    Ord,
    Hash,
    PartialOrd,
    Copy,
    Clone,
    PostgresType,
    Serialize,
    Deserialize,
    PostgresEq,
    PostgresOrd,
    PostgresHash,
)]
#[inoutfuncs]
pub struct Cnpj(i64);

impl InOutFuncs for Cnpj {
    fn input(_input: &CStr) -> Self {
        let s = _input.to_str().unwrap();
        let n = s.parse::<i64>().unwrap();

        if n < 100 || n > 99999999999999 {
            pgrx::error!("CNPJ must be between 100 and 99999999999999.");
        }

        let dv = n % 100;
        let digit = compute_digit(n / 100);

        if dv != digit {
            pgrx::error!("invalid check digit for CNPJ.");
        }

        Cnpj(n)
    }

    fn output(&self, buffer: &mut StringInfo) {
        let mut group: [i64; 5] = [0; 5];

        // XX.XXX.XXX/YYYY-ZZ
        group[0] = (self.0 / 1000000000000) % 100;
        group[1] = (self.0 / 1000000000) % 1000;
        group[2] = (self.0 / 1000000) % 1000;
        group[3] = (self.0 / 100) % 10000;
        group[4] = self.0 % 100;

        buffer.push_str(&format!(
            "{:02}.{:03}.{:03}/{:04}-{:02}",
            group[0], group[1], group[2], group[3], group[4]
        ));
    }
}
