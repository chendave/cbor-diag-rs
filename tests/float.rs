#[macro_use]
extern crate indoc;
#[macro_use]
extern crate pretty_assertions;

extern crate cbor_diag;
extern crate half;

use cbor_diag::{FloatWidth, Value};
use std::f64::{INFINITY, NAN, NEG_INFINITY};

#[macro_use]
mod utils;

testcases! {
    mod unknown {
        zero(diag2value, value2diag) {
            Value::Float {
                value: 0.0,
                bitwidth: FloatWidth::Unknown,
            },
            "0.0"
        }

        one(diag2value, value2diag) {
            Value::Float {
                value: 1.0,
                bitwidth: FloatWidth::Unknown,
            },
            "1.0"
        }

        half(diag2value, value2diag) {
            Value::Float {
                value: 0.5,
                bitwidth: FloatWidth::Unknown,
            },
            "0.5"
        }

        infinity(diag2value, value2diag) {
            Value::Float {
                value: INFINITY,
                bitwidth: FloatWidth::Unknown,
            },
            "Infinity"
        }

        neg_infinity(diag2value, value2diag) {
            Value::Float {
                value: NEG_INFINITY,
                bitwidth: FloatWidth::Unknown,
            },
            "-Infinity"
        }

        nan(value2diag) {
            Value::Float {
                value: NAN,
                bitwidth: FloatWidth::Unknown,
            },
            "NaN"
        }
    }

    mod f16 {
        zero {
            Value::Float {
                value: 0.0,
                bitwidth: FloatWidth::Sixteen,
            },
            "0.0_1",
            indoc!("
                f9 0000 # float(0.0)
            ")
        }

        one {
            Value::Float {
                value: 1.0,
                bitwidth: FloatWidth::Sixteen,
            },
            "1.0_1",
            indoc!("
                f9 3c00 # float(1.0)
            ")
        }

        half {
            Value::Float {
                value: 0.5,
                bitwidth: FloatWidth::Sixteen,
            },
            "0.5_1",
            indoc!("
                f9 3800 # float(0.5)
            ")
        }

        infinity {
            Value::Float {
                value: INFINITY,
                bitwidth: FloatWidth::Sixteen,
            },
            "Infinity_1",
            indoc!("
                f9 7c00 # float(Infinity)
            ")
        }

        neg_infinity {
            Value::Float {
                value: NEG_INFINITY,
                bitwidth: FloatWidth::Sixteen,
            },
            "-Infinity_1",
            indoc!("
                f9 fc00 # float(-Infinity)
            ")
        }

        nan(value2diag, value2hex) {
            Value::Float {
                value: NAN,
                bitwidth: FloatWidth::Sixteen,
            },
            "NaN_1",
            indoc!("
                f9 7e00 # float(NaN)
            ")
        }
    }

    mod f32 {
        zero {
            Value::Float {
                value: 0.0,
                bitwidth: FloatWidth::ThirtyTwo,
            },
            "0.0_2",
            indoc!("
                fa 00000000 # float(0.0)
            ")
        }

        one {
            Value::Float {
                value: 1.0,
                bitwidth: FloatWidth::ThirtyTwo,
            },
            "1.0_2",
            indoc!("
                fa 3f800000 # float(1.0)
            ")
        }

        half {
            Value::Float {
                value: 0.5,
                bitwidth: FloatWidth::ThirtyTwo,
            },
            "0.5_2",
            indoc!("
                fa 3f000000 # float(0.5)
            ")
        }

        infinity {
            Value::Float {
                value: INFINITY,
                bitwidth: FloatWidth::ThirtyTwo,
            },
            "Infinity_2",
            indoc!("
                fa 7f800000 # float(Infinity)
            ")
        }

        neg_infinity {
            Value::Float {
                value: NEG_INFINITY,
                bitwidth: FloatWidth::ThirtyTwo,
            },
            "-Infinity_2",
            indoc!("
                fa ff800000 # float(-Infinity)
            ")
        }

        nan(value2diag, value2hex) {
            Value::Float {
                value: NAN,
                bitwidth: FloatWidth::ThirtyTwo,
            },
            "NaN_2",
            indoc!("
                fa 7fc00000 # float(NaN)
            ")
        }
    }

    mod f64 {
        zero {
            Value::Float {
                value: 0.0,
                bitwidth: FloatWidth::SixtyFour,
            },
            "0.0_3",
            indoc!("
                fb 0000000000000000 # float(0.0)
            ")
        }

        one {
            Value::Float {
                value: 1.0,
                bitwidth: FloatWidth::SixtyFour,
            },
            "1.0_3",
            indoc!("
                fb 3ff0000000000000 # float(1.0)
            ")
        }

        half {
            Value::Float {
                value: 0.5,
                bitwidth: FloatWidth::SixtyFour,
            },
            "0.5_3",
            indoc!("
                fb 3fe0000000000000 # float(0.5)
            ")
        }

        infinity {
            Value::Float {
                value: f64::INFINITY,
                bitwidth: FloatWidth::SixtyFour,
            },
            "Infinity_3",
            indoc!("
                fb 7ff0000000000000 # float(Infinity)
            ")
        }

        neg_infinity {
            Value::Float {
                value: f64::NEG_INFINITY,
                bitwidth: FloatWidth::SixtyFour,
            },
            "-Infinity_3",
            indoc!("
                fb fff0000000000000 # float(-Infinity)
            ")
        }

        nan(value2diag, value2hex) {
            Value::Float {
                value: f64::NAN,
                bitwidth: FloatWidth::SixtyFour,
            },
            "NaN_3",
            indoc!("
                fb 7ff8000000000000 # float(NaN)
            ")
        }
    }
}
