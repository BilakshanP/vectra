pub enum BaseUnit {
    Length,
    Mass,
    Time,
    ElectricCurrent,
    Temperature,
    AmountOfSubstance,
    LuminousIntensity,
}

impl BaseUnit {
    pub fn get_symbol(&self) -> &str {
        match self {
            BaseUnit::Length => "L",
            BaseUnit::Mass => "M",
            BaseUnit::Time => "T",
            BaseUnit::ElectricCurrent => "I",
            BaseUnit::Temperature => "Θ",
            BaseUnit::AmountOfSubstance => "N",
            BaseUnit::LuminousIntensity => "J",
        }
    }
}

#[derive(PartialEq)]
pub enum UnitPrefix {
    Yotta = 24,
    Zetta = 21,
    Exa = 18,
    Peta = 15,
    Tera = 12,
    Giga = 9,
    Mega = 6,
    Kilo = 3,
    Hecto = 2,
    Deca = 1,
    None = 0,
    Deci = -1,
    Centi = -2,
    Milli = -3,
    Micro = -6,
    Nano = -9,
    Pico = -12,
    Femto = -15,
    Atto = -18,
    Zepto = -21,
    Yocto = -24,
}

impl UnitPrefix {
    pub fn get_symbol(&self) -> &str {
        match self {
            UnitPrefix::Yotta => "Y",
            UnitPrefix::Zetta => "Z",
            UnitPrefix::Exa => "E",
            UnitPrefix::Peta => "P",
            UnitPrefix::Tera => "T",
            UnitPrefix::Giga => "G",
            UnitPrefix::Mega => "M",
            UnitPrefix::Kilo => "k",
            UnitPrefix::Hecto => "h",
            UnitPrefix::Deca => "da",
            UnitPrefix::None => "",
            UnitPrefix::Deci => "d",
            UnitPrefix::Centi => "c",
            UnitPrefix::Milli => "m",
            UnitPrefix::Micro => "μ",
            UnitPrefix::Nano => "n",
            UnitPrefix::Pico => "p",
            UnitPrefix::Femto => "f",
            UnitPrefix::Atto => "a",
            UnitPrefix::Zepto => "z",
            UnitPrefix::Yocto => "y",
        }
    }
}

pub struct DimensionalUnit {
    base: BaseUnit,
    prefix: UnitPrefix,
    power: i8,
}

pub struct Unit {
    values: [DimensionalUnit; 7],
}

impl Default for Unit {
    fn default() -> Self {
        Unit {
            values: [
                DimensionalUnit {
                    base: BaseUnit::Length,
                    prefix: UnitPrefix::None,
                    power: 0,
                },
                DimensionalUnit {
                    base: BaseUnit::Mass,
                    prefix: UnitPrefix::None,
                    power: 0,
                },
                DimensionalUnit {
                    base: BaseUnit::Time,
                    prefix: UnitPrefix::None,
                    power: 0,
                },
                DimensionalUnit {
                    base: BaseUnit::ElectricCurrent,
                    prefix: UnitPrefix::None,
                    power: 0,
                },
                DimensionalUnit {
                    base: BaseUnit::Temperature,
                    prefix: UnitPrefix::None,
                    power: 0,
                },
                DimensionalUnit {
                    base: BaseUnit::AmountOfSubstance,
                    prefix: UnitPrefix::None,
                    power: 0,
                },
                DimensionalUnit {
                    base: BaseUnit::LuminousIntensity,
                    prefix: UnitPrefix::None,
                    power: 0,
                },
            ],
        }
    }
}

impl Unit {
    pub fn new() -> Self {
        Unit::default()
    }

    pub fn new_from_raw(
        length: DimensionalUnit,
        mass: DimensionalUnit,
        time: DimensionalUnit,
        electric_current: DimensionalUnit,
        temperature: DimensionalUnit,
        amount_of_substance: DimensionalUnit,
        luminous_intensity: DimensionalUnit,
    ) -> Self {
        Unit {
            values: [
                length,
                mass,
                time,
                electric_current,
                temperature,
                amount_of_substance,
                luminous_intensity,
            ],
        }
    }

    pub fn print(&self) {
        let sup = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];

        for i in 0..7 {
            let mut symbol = self.values[i].base.get_symbol().to_string();
            if self.values[i].prefix != UnitPrefix::None {
                symbol.push_str(self.values[i].prefix.get_symbol());
            }
            if self.values[i].power != 0 {
                symbol.push(sup[self.values[i].power as usize]);
            }
            print!("{}", symbol);
        }
    }
}
