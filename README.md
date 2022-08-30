# coffee_maker
Emulation of my coffee machine in Rust (learning purpose only)

## How it works

This coffee machine can make 2 types of coffee (expresso and american) and also can pour hot water.

For this to proper function we need to suply with coffee or water if there's not enough, or to empty the dump if it's full.

## Technical requisites

### States
The coffee machine has 2 possible states:
- `Ready`: Everything is ready to brew some drinks.
- `ActionRequired`: A maintenance action is required to continue brewing drinks.

### Actions
The brewing actions are the following:
- `ExpressoCoffee`: uses 9gr of coffee and 40ml of water.
- `AmericanCoffee`: uses 7gr of coffee and 60ml of water.
- `HotWater`: uses 0gr of coffee and 75ml of water.
  
Note: All brewing action produces the same amount of waste as their amount of coffee consumption.

The maintenance actions are the following:
- `FillWater`: Fills 100% of the water deposit capacity, which is 255ml.
- `FillCoffee`: Fills 100% of the coffee deposit capacity, which is 100gr.
- `EmptyDump`: Empties 100% of the waste dump, which is 50gr.

Note: After doing any action, the deposits are checked to update the state.

### State transition after action
For the machine to be at the `Ready` state:
- The water deposit must have enough water to make the most water-consuming drink.
- The coffee deposit must have enough coffee to make the most coffee-consuming drink.
- The waste dump must have enough room to make the most coffee-wasting drink.

If any of those constraints is not met, the machine will be at `ActionRequired` state.
