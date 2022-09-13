use eyre::Result;
use inquire::Select;
use std::fmt::Display;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use strum_macros::EnumString;

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Ready,
    ActionRequired,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, EnumString, EnumIter)]
enum BrewAction {
    ExpressoCoffee,
    AmericanCoffee,
    HotWater,
}

impl Display for BrewAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct BrewConsumption {
    coffee: u8,
    water: u8,
}

#[derive(Debug, EnumString, EnumIter)]
enum MaintenanceAction {
    FillWater,
    FillCoffee,
    EmptyDump,
}

impl Display for MaintenanceAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Deposit {
    current_load: u8,
    max_load: u8,
}

pub struct CoffeeMachine {
    coffee_deposit: Deposit,
    water_deposit: Deposit,
    waste_dump: Deposit,
    expresso_coffee_consumption: BrewConsumption,
    american_coffee_consumption: BrewConsumption,
    hot_water_consumption: BrewConsumption,
    current_state: State,
}

impl CoffeeMachine {
    pub fn new() -> Self {
        let new_coffee_machine = Self {
            coffee_deposit: Deposit {
                current_load: 0,
                max_load: 100,
            },
            water_deposit: Deposit {
                current_load: 0,
                max_load: 255,
            },
            waste_dump: Deposit {
                current_load: 0,
                max_load: 50,
            },
            expresso_coffee_consumption: BrewConsumption {
                coffee: 9,
                water: 40,
            },
            american_coffee_consumption: BrewConsumption {
                coffee: 7,
                water: 60,
            },
            hot_water_consumption: BrewConsumption {
                coffee: 0,
                water: 75,
            },
            current_state: State::ActionRequired,
        };

        new_coffee_machine.print_full_state();

        new_coffee_machine
    }

    pub fn run(&mut self) {
        loop {
            match self.submit_action() {
                Ok(_) => continue,
                Err(e) => {
                    println!("{}", e);
                    println!("Exiting application...");
                    break;
                }
            }
        }
    }

    fn calculate_max_required_coffee(&self) -> u8 {
        vec![
            self.expresso_coffee_consumption.coffee,
            self.american_coffee_consumption.coffee,
            self.hot_water_consumption.coffee,
        ]
        .iter()
        .max()
        .unwrap_or(&0)
        .to_owned()
    }

    fn calculate_max_required_water(&self) -> u8 {
        vec![
            self.expresso_coffee_consumption.water,
            self.american_coffee_consumption.water,
            self.hot_water_consumption.water,
        ]
        .iter()
        .max()
        .unwrap_or(&0)
        .to_owned()
    }

    fn is_coffe_deposit_empty(&self) -> bool {
        self.coffee_deposit.current_load < self.calculate_max_required_coffee()
    }

    fn is_water_deposit_empty(&self) -> bool {
        self.water_deposit.current_load < self.calculate_max_required_water()
    }

    fn is_waste_dump_full(&self) -> bool {
        self.waste_dump.current_load
            > self.waste_dump.max_load - self.calculate_max_required_coffee()
    }

    fn fill_water_deposit(&mut self) {
        self.water_deposit.current_load = self.water_deposit.max_load;
        self.check_state();
    }

    fn fill_coffee_deposit(&mut self) {
        self.coffee_deposit.current_load = self.coffee_deposit.max_load;
        self.check_state();
    }

    fn empty_waste_dump(&mut self) {
        self.waste_dump.current_load = 0;
        self.check_state();
    }

    fn brew_expresso_coffee(&mut self) {
        self.coffee_deposit.current_load -= self.expresso_coffee_consumption.coffee;
        self.water_deposit.current_load -= self.expresso_coffee_consumption.water;
        self.waste_dump.current_load += self.expresso_coffee_consumption.coffee;
        self.check_state();
    }

    fn brew_american_coffee(&mut self) {
        self.coffee_deposit.current_load -= self.american_coffee_consumption.coffee;
        self.water_deposit.current_load -= self.american_coffee_consumption.water;
        self.waste_dump.current_load += self.american_coffee_consumption.coffee;
        self.check_state();
    }

    fn brew_hot_water(&mut self) {
        self.coffee_deposit.current_load -= self.hot_water_consumption.coffee;
        self.water_deposit.current_load -= self.hot_water_consumption.water;
        self.waste_dump.current_load += self.hot_water_consumption.coffee;
        self.check_state();
    }

    fn check_state(&mut self) {
        self.current_state = if self.is_coffe_deposit_empty()
            || self.is_water_deposit_empty()
            || self.is_waste_dump_full()
        {
            State::ActionRequired
        } else {
            State::Ready
        };

        self.print_full_state();
    }

    fn print_full_state(&self) {
        println!("--------------------------------");

        if self.is_coffe_deposit_empty() {
            print!("WARNING [EMPTY]: ");
        }
        println!(
            "Coffee deposit {}gr out of {}gr.",
            self.coffee_deposit.current_load, self.coffee_deposit.max_load
        );

        if self.is_water_deposit_empty() {
            print!("WARNING [EMPTY]: ");
        }
        println!(
            "Water deposit {}cl out of {}cl.",
            self.water_deposit.current_load, self.water_deposit.max_load
        );

        if self.is_waste_dump_full() {
            print!("WARNING [FULL]: ");
        }
        println!(
            "Waste deposit {}gr out of {}gr.",
            self.waste_dump.current_load, self.waste_dump.max_load
        );

        println!("Current state: {}.", self.current_state);
    }

    fn actions_from_current_state(&self) -> Result<String> {
        let action = match self.current_state {
            State::Ready => {
                let options = BrewAction::iter().map(|a| a.to_string()).collect();

                Select::new("Select your brew action", options).prompt()?
            }
            State::ActionRequired => {
                let options = MaintenanceAction::iter().map(|a| a.to_string()).collect();

                Select::new("WARNING: Maintenance action required!", options).prompt()?
            }
        };
        Ok(action)
    }

    fn submit_action(&mut self) -> Result<()> {
        let action = self.actions_from_current_state()?;

        match self.current_state {
            State::Ready => match BrewAction::from_str(&action)? {
                BrewAction::ExpressoCoffee => self.brew_expresso_coffee(),
                BrewAction::AmericanCoffee => self.brew_american_coffee(),
                BrewAction::HotWater => self.brew_hot_water(),
            },
            State::ActionRequired => match MaintenanceAction::from_str(&action)? {
                MaintenanceAction::FillWater => self.fill_water_deposit(),
                MaintenanceAction::FillCoffee => self.fill_coffee_deposit(),
                MaintenanceAction::EmptyDump => self.empty_waste_dump(),
            },
        };
        Ok(())
    }
}

// --------- Testing --------- //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let coffee_maker = CoffeeMachine::new();
        assert_eq!(coffee_maker.current_state, State::ActionRequired);
        assert_eq!(
            coffee_maker.coffee_deposit,
            Deposit {
                current_load: 0,
                max_load: 100,
            }
        );
        assert_eq!(
            coffee_maker.coffee_deposit,
            Deposit {
                current_load: 0,
                max_load: 100,
            }
        );
        assert_eq!(
            coffee_maker.coffee_deposit,
            Deposit {
                current_load: 0,
                max_load: 100,
            }
        );

        assert_eq!(
            coffee_maker.water_deposit,
            Deposit {
                current_load: 0,
                max_load: 255,
            }
        );
        assert_eq!(
            coffee_maker.waste_dump,
            Deposit {
                current_load: 0,
                max_load: 50,
            }
        );
        assert_eq!(
            coffee_maker.expresso_coffee_consumption,
            BrewConsumption {
                coffee: 9,
                water: 40,
            }
        );
        assert_eq!(
            coffee_maker.american_coffee_consumption,
            BrewConsumption {
                coffee: 7,
                water: 60,
            }
        );
        assert_eq!(
            coffee_maker.hot_water_consumption,
            BrewConsumption {
                coffee: 0,
                water: 75,
            }
        );
    }

    #[test]
    fn test_calculate_max_required_coffee() {
        let coffee_maker = CoffeeMachine::new();

        assert_eq!(
            coffee_maker.calculate_max_required_coffee(),
            coffee_maker.expresso_coffee_consumption.coffee
        )
    }

    #[test]
    fn test_calculate_max_required_water() {
        let coffee_maker = CoffeeMachine::new();

        assert_eq!(
            coffee_maker.calculate_max_required_water(),
            coffee_maker.hot_water_consumption.water
        )
    }

    #[test]
    fn test_is_waste_dump_full() {
        let mut coffee_maker = CoffeeMachine::new();

        coffee_maker.waste_dump.current_load =
            coffee_maker.waste_dump.max_load - coffee_maker.calculate_max_required_coffee();
        assert_eq!(coffee_maker.is_waste_dump_full(), false);

        coffee_maker.waste_dump.current_load += 1;
        assert_eq!(coffee_maker.is_waste_dump_full(), true);
    }

    #[test]
    fn test_is_water_deposit_empty() {
        let mut coffee_maker = CoffeeMachine::new();

        coffee_maker.water_deposit.current_load = coffee_maker.calculate_max_required_water();
        assert_eq!(coffee_maker.is_water_deposit_empty(), false);

        coffee_maker.water_deposit.current_load -= 1;
        assert_eq!(coffee_maker.is_water_deposit_empty(), true);
    }

    #[test]
    fn test_is_coffe_deposit_empty() {
        let mut coffee_maker = CoffeeMachine::new();

        coffee_maker.coffee_deposit.current_load = coffee_maker.calculate_max_required_coffee();
        assert_eq!(coffee_maker.is_coffe_deposit_empty(), false);

        coffee_maker.coffee_deposit.current_load -= 1;
        assert_eq!(coffee_maker.is_coffe_deposit_empty(), true);
    }

    #[test]
    fn test_check_state() {
        let mut coffee_maker = CoffeeMachine::new();

        // Brand-new
        coffee_maker.check_state();
        assert_eq!(coffee_maker.current_state, State::ActionRequired);

        coffee_maker.fill_coffee_deposit();
        coffee_maker.fill_water_deposit();
        assert_eq!(coffee_maker.current_state, State::Ready);

        // Coffee deposit
        coffee_maker.coffee_deposit.current_load = coffee_maker.calculate_max_required_coffee() - 1;
        coffee_maker.check_state();
        assert_eq!(coffee_maker.current_state, State::ActionRequired);

        coffee_maker.fill_coffee_deposit();
        assert_eq!(coffee_maker.current_state, State::Ready);

        // Water deposit
        coffee_maker.water_deposit.current_load = coffee_maker.calculate_max_required_water() - 1;
        coffee_maker.check_state();
        assert_eq!(coffee_maker.current_state, State::ActionRequired);

        coffee_maker.fill_water_deposit();
        assert_eq!(coffee_maker.current_state, State::Ready);

        // Waste dump
        coffee_maker.waste_dump.current_load =
            coffee_maker.waste_dump.max_load - coffee_maker.calculate_max_required_coffee() + 1;
        coffee_maker.check_state();
        assert_eq!(coffee_maker.current_state, State::ActionRequired);

        coffee_maker.empty_waste_dump();
        assert_eq!(coffee_maker.current_state, State::Ready);
    }

    #[test]
    fn test_brew_expresso_coffee() {
        let mut coffee_maker = CoffeeMachine::new();

        coffee_maker.coffee_deposit.current_load =
            (coffee_maker.expresso_coffee_consumption.coffee * 2) - 1;
        coffee_maker.water_deposit.current_load =
            (coffee_maker.expresso_coffee_consumption.water * 2) - 1;
        coffee_maker.waste_dump.current_load = coffee_maker.waste_dump.max_load
            - (coffee_maker.expresso_coffee_consumption.coffee * 2)
            + 1;
        coffee_maker.check_state();
        assert_eq!(coffee_maker.current_state, State::Ready);

        coffee_maker.brew_expresso_coffee();
        assert_eq!(coffee_maker.current_state, State::ActionRequired);

        coffee_maker.fill_coffee_deposit();
        assert_eq!(coffee_maker.current_state, State::ActionRequired);

        coffee_maker.fill_water_deposit();
        assert_eq!(coffee_maker.current_state, State::ActionRequired);

        coffee_maker.empty_waste_dump();
        assert_eq!(coffee_maker.current_state, State::Ready);
    }

    #[test]
    fn test_brew_american_coffee() {
        let mut coffee_maker = CoffeeMachine::new();

        coffee_maker.coffee_deposit.current_load =
            (coffee_maker.american_coffee_consumption.coffee * 2) - 1;
        coffee_maker.water_deposit.current_load =
            (coffee_maker.american_coffee_consumption.water * 2) - 1;
        coffee_maker.waste_dump.current_load = coffee_maker.waste_dump.max_load
            - (coffee_maker.american_coffee_consumption.coffee * 2)
            + 1;
        coffee_maker.check_state();
        assert_eq!(coffee_maker.current_state, State::Ready);

        coffee_maker.brew_american_coffee();
        assert_eq!(coffee_maker.current_state, State::ActionRequired);

        coffee_maker.fill_coffee_deposit();
        assert_eq!(coffee_maker.current_state, State::ActionRequired);

        coffee_maker.fill_water_deposit();
        assert_eq!(coffee_maker.current_state, State::ActionRequired);

        coffee_maker.empty_waste_dump();
        assert_eq!(coffee_maker.current_state, State::Ready);
    }

    #[test]
    fn test_brew_hot_water() {
        let mut coffee_maker = CoffeeMachine::new();

        coffee_maker.coffee_deposit.current_load = coffee_maker.calculate_max_required_coffee();
        coffee_maker.water_deposit.current_load =
            (coffee_maker.hot_water_consumption.water * 2) - 1;
        coffee_maker.waste_dump.current_load =
            coffee_maker.waste_dump.max_load - coffee_maker.calculate_max_required_coffee();
        coffee_maker.check_state();
        assert_eq!(coffee_maker.current_state, State::Ready);

        coffee_maker.brew_hot_water();
        assert_eq!(coffee_maker.current_state, State::ActionRequired);

        coffee_maker.fill_water_deposit();
        assert_eq!(coffee_maker.current_state, State::Ready);
    }
}
