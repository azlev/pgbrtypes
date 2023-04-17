use core::ffi::CStr;
use pgrx::prelude::*;
use pgrx::{InOutFuncs, StringInfo};
use serde::{Deserialize, Serialize};

fn compute_digit(mut cpf: i64) -> i64 {
    let mut digits: [i64; 9] = [0; 9];

    for i in (0..=8).rev() {
        digits[i] = cpf % 10;
        cpf = cpf / 10
    }

    let mut d1 = (10 * digits[0])
        + (9 * digits[1])
        + (8 * digits[2])
        + (7 * digits[3])
        + (6 * digits[4])
        + (5 * digits[5])
        + (4 * digits[6])
        + (3 * digits[7])
        + (2 * digits[8]);
    d1 = 11 - (d1 % 11);
    if d1 > 9 {
        d1 = 0;
    }
    let mut d2 = (11 * digits[0])
        + (10 * digits[1])
        + (9 * digits[2])
        + (8 * digits[3])
        + (7 * digits[4])
        + (6 * digits[5])
        + (5 * digits[6])
        + (4 * digits[7])
        + (3 * digits[8])
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
pub struct Cpf(i64);

impl InOutFuncs for Cpf {
    fn input(_input: &CStr) -> Self {
        let s = _input.to_str().unwrap();
        let n = s.parse::<i64>().unwrap();

        if n < 100 || n > 99999999999 {
            pgrx::error!("CPF must be between 100 and 99999999999.");
        }

        let dv = n % 100;
        let digit = compute_digit(n / 100);

        if dv != digit {
            pgrx::error!("invalid check digit for CPF.");
        }
        for i in 1..10 {
            if n == i * 11111111111 {
                pgrx::error!("All CPF digits should not be equal.");
            }
        }

        Cpf(n)
    }

    fn output(&self, buffer: &mut StringInfo) {
        let mut group: [i64; 4] = [0; 4];

        // 999.999.999-99
        group[0] = (self.0 / 100000000) % 1000;
        group[1] = (self.0 / 100000) % 1000;
        group[2] = (self.0 / 100) % 1000;
        group[3] = self.0 % 100;

        buffer.push_str(&format!(
            "{:3}.{:03}.{:03}-{:02}",
            group[0], group[1], group[2], group[3]
        ));
    }
}
