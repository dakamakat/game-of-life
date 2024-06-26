use wasm_bindgen_test::wasm_bindgen_test;
use wasm_game_of_life::universe::Universe;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
mod test {
    use wasm_bindgen_test::console_log;

    use super::*;

    #[test]
    pub fn input_spaceship() -> Universe {
        let mut universe = Universe::new(0 , 100 , 80);
        universe.set_width(6);
        universe.set_height(6);
        universe.set_cells_borrow(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
        console_log!("{}", universe);
        universe
    }

    #[test]
    pub fn expected_spaceship() -> Universe {
        let mut universe = Universe::new(0 , 100 , 80);
        universe.set_width(6);
        universe.set_height(6);
        universe.set_cells_borrow(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
        console_log!("{}", universe);
        universe
    }

    #[wasm_bindgen_test]
    pub fn test_tick() {
        // Let's create a smaller Universe with a small spaceship to test!
        let mut input_universe = input_spaceship();

        // This is what our spaceship should look like
        // after one tick in our universe.
        let expected_universe = expected_spaceship();

        // Call `tick` and then see if the cells in the `Universe`s are the same.
        input_universe.tick();

        console_log!("{}", input_universe);

        assert_eq!(input_universe.get_cells(), expected_universe.get_cells());
    }
}
