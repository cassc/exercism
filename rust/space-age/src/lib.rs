#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s)
    }
}

pub trait Planet {
    const SCALE: f64;
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / Self::SCALE / 31557600.0
    }
}

macro_rules! planet_scales {
    ($($name:ident -> $scale:literal),+) => {
        $(
            pub struct $name;

            impl Planet for $name {
                const SCALE: f64 = $scale;
            }
        )+
    }
}

planet_scales!{
    Mercury -> 0.2408467,
    Venus -> 0.61519726,
    Earth -> 1.0,
    Mars -> 1.8808158,
    Jupiter -> 11.862615,
    Saturn -> 29.447498,
    Uranus -> 84.016846,
    Neptune -> 164.79132
}
