trait GameOfLife {
    fn lives(self, idx: usize, neighbors: u8, ruleset: u128) -> bool;
    fn step(self, neighbors: u8, ruleset: u128) -> Self;
}

impl GameOfLife for [bool; 32] {
    fn lives(self, idx: usize, neighbors: u8, ruleset: u128) -> bool {
        let pointer = idx as i8 - neighbors as i8;
        let counter = (2 * neighbors) as i8;

        let vitality = calc_vitality(self, pointer, counter);

        // Check if the current position in the ruleset given by the vitality value is true
        if ruleset_to_array(ruleset, 0)[vitality as usize] {
            return true;
        } else {
            return false;
        }
    }

    fn step(self, neighbors: u8, ruleset: u128) -> Self {
        // Initialize new array and set every value to false
        let resultset: [bool; 32] = [false; 32];

        iter(self, resultset, 0, neighbors, ruleset)
    }
}

/*
 * Simple function to calculate the n-th power of a given number
 */
fn pow(z: i16, e: i16) -> i16 {
    if e == 0 {
        1
    } else if e == 1 {
        z
    } else {
        z * pow(z, e - 1)
    }
}

/*
 * Calculates the vital value which is used to determine the bool value at the position of the
 * pointer from the ruleset.
 *
 * arr: [bool; 32] ...array which is to be checked
 * mut pointer: i8 ...starts with the first value representing the current position in the array to
 *                    be calculated and ends at -1
 * counter: i8     ...represents the power of the current cell starting with the highest (since the
 *                    array is read from left to right) and at the same time forms termination 
 *                    condition of the recursion
 */
fn calc_vitality(arr: [bool; 32], mut pointer: i8, counter: i8) -> i16 {
    // At first check if pointer is negative, because modulo operation in rust with negative
    // numbers doesn't work in a mathematical way
    if pointer < 0 {
        // ..if true, then add 32 to get the correct field of the array
        pointer = pointer + 32;
    } else {
        // ..if false, then calculate modulo 32 for the same result
        pointer = pointer % 32;
    }

    // This is the main termination condition
    if counter < 0 {
        return 0;
    } else {
        let mut vitality = 0;

        // If the current field in arr is true..
        if arr[pointer as usize] {
            // ..then add the power of counter (base 2) and the next iteration of calc_vitality
            vitality += pow(2, counter as i16) + calc_vitality(arr, pointer + 1, counter - 1);
        } else {
            // ..if not, then there is nothing to do, just skip this step and move to the next one
            return calc_vitality(arr, pointer + 1, counter - 1);
        }

        vitality
    }
}

/*
 * Simple iteration over the given "cell-universe"
 */
fn iter(arr: [bool; 32], mut resultset: [bool; 32], idx: usize, neighbors: u8, ruleset: u128) -> [bool; 32] {
    if idx < 32 {
        // Check life state in next gen..
        if arr.lives(idx, neighbors, ruleset) {
            // ..and write the calculated values into the result array
            resultset[idx] = true;
        } else {
            resultset[idx] = false;
        }

        iter(arr, resultset, idx + 1, neighbors, ruleset)
    } else {
        resultset
    }
}

/*
 * Converts the ruleset number into an array with binary values to compare vitality value later
 */
fn ruleset_to_array(ruleset: u128, idx: usize) -> [bool; 128] {
    // Static count to 128
    if idx == 128 {
        // Return empty array if counter reached the end
        [false; 128] 
    } else {
        // Declare new array and write next iteration into it
        // Divide ruleset by 2 (which slices the floats) and increase counter (idx) by 1
        // This will be declared backwards, beginning by idx=31 to idx=0
        let mut array = ruleset_to_array(ruleset / 2, idx + 1); 

        // Get correct binary for the given pointer idx..
        if ruleset % 2 == 1 {
            // ..and write the correct value
            array[idx] = true;
        } else {
            array[idx] = false;
        }

        array 
    }
}

