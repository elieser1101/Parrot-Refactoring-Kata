static BASE_SPEED: f32 = 12.0;
static LOAD_FACTOR:f32 = 9.0;


struct Parrot {
    number_of_coconuts: usize,
    voltage: f32,
    nailed: bool,
}

//require the speed method for all kind of parrots
trait ParrotTrait {
    fn speed(&self) -> Result<f32, &str>;
}

struct EuropeanParrot {
}

impl ParrotTrait for EuropeanParrot {
    fn speed(&self) -> Result<f32, &str> {
        Ok(BASE_SPEED)
    }
}

struct AfricanParrot {
    parrot: Parrot
}

impl ParrotTrait for AfricanParrot {
    fn speed(&self) -> Result<f32, &str> {
        let african_speed = BASE_SPEED - LOAD_FACTOR * self.parrot.number_of_coconuts as f32;
        if african_speed > 0.0 { Ok(african_speed) } else { Ok(0.0)}
    }
}

struct NorwegianblueParrot {
    parrot: Parrot
}

impl ParrotTrait for NorwegianblueParrot {
    fn speed(&self) -> Result<f32, &str> {
        if self.parrot.nailed == true {
            Ok(0.0)
        }
        else {
            //compute_base_speed_for_voltage(voltage: f32) -> f32 {
            let fixed_base_speed = 24.0;
            let base_speed_for_voltage = self.parrot.voltage * BASE_SPEED;
            if base_speed_for_voltage < fixed_base_speed {
                Ok(base_speed_for_voltage)
            }
            else {
                Ok(fixed_base_speed)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn european_parrot_speed() {
        let parrot = EuropeanParrot {};
        assert_eq!(parrot.speed().unwrap(), 12.0);
    }

    #[test]
    fn african_parrot_speed_with_one_coconut() {
        let parrot = AfricanParrot { parrot : Parrot { number_of_coconuts: 1,
                              voltage: 0.0,
                              nailed: false }};
        assert_eq!(parrot.speed().unwrap(), 3.0);
    }

    #[test]
    fn african_parrot_speed_with_two_coconut() {
        let parrot = AfricanParrot { parrot : Parrot { number_of_coconuts: 2,
                              voltage: 0.0,
                              nailed: false }};
        assert_eq!(parrot.speed().unwrap(), 0.0);
    }

    #[test]
    fn african_parrot_speed_with_no_coconut() {
        let parrot = AfricanParrot { parrot : Parrot { number_of_coconuts: 0,
                              voltage: 0.0,
                              nailed: false }};
        assert_eq!(parrot.speed().unwrap(), 12.0);
    }

    #[test]
    fn nailed_norwegian_blue_parrot() {
        let parrot = NorwegianblueParrot { parrot: Parrot { number_of_coconuts: 0,
                              voltage: 1.5,
                              nailed: true }};
        assert_eq!(parrot.speed().unwrap(), 0.0);
    }

    #[test]
    fn not_nailed_norwegian_blue_parrot() {
        let parrot = NorwegianblueParrot { parrot: Parrot { number_of_coconuts: 0,
                              voltage: 1.5,
                              nailed: false }};
        assert_eq!(parrot.speed().unwrap(), 18.0);
    }

    #[test]
    fn not_nailed_norwegian_blue_parrot_with_high_voltage() {
        let parrot = NorwegianblueParrot { parrot: Parrot { number_of_coconuts: 0,
                              voltage: 4.0,
                              nailed: false }};
        assert_eq!(parrot.speed().unwrap(), 24.0);
    }
}
