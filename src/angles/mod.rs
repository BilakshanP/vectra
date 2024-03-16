use std::f64::consts::PI;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub enum AngleTypes {
    Deg(f64),
    Rad(f64),
}

#[derive(Default)]
pub struct Angle {
    deg: f64,
    rad: f64,
}

impl Angle {
    pub fn get_deg(&self) -> f64 {
        self.deg
    }

    pub fn get_rad(&self) -> f64 {
        self.rad
    }
}

impl Angle {
    pub fn new(val: AngleTypes) -> Angle {
        match val {
            AngleTypes::Deg(deg) => Angle {
                deg,
                rad: deg * PI / 180.0,
            },
            AngleTypes::Rad(rad) => Angle {
                deg: rad * 180.0 / PI,
                rad,
            },
        }
    }

    pub fn new_rad(rad: f64) -> Angle {
        Angle {
            deg: rad * 180.0 / PI,
            rad,
        }
    }

    pub fn new_deg(deg: f64) -> Angle {
        Angle {
            deg,
            rad: deg * PI / 180.0,
        }
    }
}

impl Debug for Angle {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "<{}deg {}rad>", self.deg, self.rad)
    }
}

impl Display for Angle {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}deg", self.deg)
    }
}
