mod coffee_machine;
use coffee_machine::CoffeeMachine;

fn main() {
    let mut coffee_maker = CoffeeMachine::new();
    loop {
        let state = coffee_maker.get_current_state();
        let next_action = coffee_maker.actions_from_state(state);
        coffee_maker.submit_action(next_action);
    }
}
