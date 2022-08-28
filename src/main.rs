mod coffee_machine;
use coffee_machine::CoffeeMachine;

fn main() {
    let mut coffee_maker = CoffeeMachine::new();
    loop {
        let next_action = coffee_maker.actions_from_current_state();
        coffee_maker.submit_action(next_action.as_str());
    }
}
