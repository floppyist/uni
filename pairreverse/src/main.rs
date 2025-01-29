/*
 * Takes a number and calculates all existent pairs.
 * 
 * Notice that a number like 123456 will return only 2 pairs.
 * This is expected, because this function is used to get the
 * correct exponent for calculation like below:
 *
 * [12 34 56] => [56 34 12]      [e]
 * 12 00 00 +     56 00 00 + (100^2)
 *    34 00 +        34 00 + (100^1)
 *       56             12   (100^0)
 */
fn get_remaining_pairs(z: u128) -> u128 {
    if z < 100 {
        0
    } else {
        // Using division on integers will allow us to get a specific slice.
        // 1234 / 100 = "12.34" => "12" 
        1 + get_remaining_pairs(z / 100)
    }
}
/*
 * Simple function which calculates exponentials.
 */
fn pow(z: u128, e: u128) -> u128 {
    if e == 0 {
        1
    } else if e == 1 {
        z
    } else {
        z * pow(z, e - 1)
    }
}

/*
 * This function takes a number (with an even amount of digits) and
 * returns the number reversed without changing the pairs itself.
 *
 * [12 34 56] => [56 34 12]
 * 
 * [12 34 56] % 100 = (56 * 100^2) = [56 00 00]
 * [12 34 56] / 100 = (1234)
 *    [12 34] % 100 = (34 * 100^1) =    [34 00]
 *    [12 34] / 100 = (12 < 100)   =       [12]
 *                                 = [56 34 12]
 */
fn pairreverse(z: u128) -> u128 {
    if z < 100 {
        z
    } else {
        z % 100 * (pow(100, get_remaining_pairs(z))) + pairreverse(z / 100)
    }
}
