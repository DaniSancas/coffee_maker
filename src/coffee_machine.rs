enum State {
    Ready,
    ActionRequired,
}

enum BrewAction {
    ExpressoCoffee = BrewConsumption(10, 40),
    AmericanCofee = BrewConsumption(7, 70),
    HotWater = BrewConsumption(0, 50),
}

enum MaintenanceAction {
    FillWater,
    FillCoffee,
    EmptyDump,
}

struct Deposit {
    CurrentLoad: u8,
    MaxLoad: u8,
}

struct CoffeeMachine {
    CoffeeDeposit: Deposit,
    WaterDeposit: Deposit,
    WasteDeposit: Deposit,
    CurrentState: State,
}

struct BrewConsumption {
    Coffee: u8,
    Water: u8,
}
