struct Coord {
    row: usize,
    col: usize,
}

#[derive(Copy,Clone,PartialEq)]
enum Square {
    U, // Kein Zugang
    Z, // Getreidelager (Ziel), nach einem Tag Vermehrung
    V, // Essen, nach einem Tag Vermehrung
    W, // Essen, nach zwei Tagen Vermehrung
    T, // Tribbles vorhanden
}

trait Tribbles {
    fn get_targets_within_days(&mut self, days: u32, start: &[Option<Coord>; 10]) -> u32;
    fn cell_is_reachable(self, coord: Coord, start: &[Option<Coord>; 10]) -> bool;
}

impl Tribbles for [[Square; 8]; 8] {
    fn get_targets_within_days(&mut self, days: u32, start: &[Option<Coord>; 10]) -> u32 {
        let array_evo = spawn_tribbles_on_map(*self, &start, 9);
        let mut mapping = create_mapping(*self, 7, 7);
        let mut array = array_evo;

        *self = get_fully_evolved_array_within_days(&mut array, &mut mapping, days);

        return count_reached_goals(*self, mapping, 7, 7) as u32;
    }

    fn cell_is_reachable(self, coord: Coord, start: &[Option<Coord>; 10]) -> bool {
        let mut array = spawn_tribbles_on_map(self, start, 9);

        if cell_is_surrounded(&mut array, coord.row, coord.col) {
            return false;
        } else {
            return true;
        }
    }
}

fn get_fully_evolved_array_within_days(array: &mut [[Square; 8]; 8], mapping: &mut [[i8; 8]; 8], days: u32) -> [[Square; 8]; 8] {
    if days == 0 {
        return *array;
    } else {
        get_fully_evolved_array_within_days(array, mapping, days - 1);

        *array = evolve_tribbles(*array, array, *mapping, 7, 7);
        *mapping = reduce_tribbled_cells(*array, mapping, 7, 7);
    }

    return *array;
}

fn reduce_tribbled_cells(array: [[Square; 8]; 8], mapping: &mut [[i8; 8]; 8], x: i8, y: i8) -> [[i8; 8]; 8] {
    if x < 0 {
        return [[0; 8]; 8];
    }

    if y < 0 {
        return reduce_tribbled_cells(array, mapping, x - 1, y + 8);
    }

    if array[x as usize][y as usize] == Square::T && mapping[x as usize][y as usize] > 1 {
        mapping[x as usize][y as usize] -= 1; 
    }

    reduce_tribbled_cells(array, mapping, x, y - 1);
    return *mapping;
}

fn count_reached_goals(array: [[Square; 8]; 8], mapping: [[i8; 8]; 8], x: i8, y: i8) -> u8 {
    if x < 0 {
        return 0;
    }

    if y < 0 {
        return count_reached_goals(array, mapping, x - 1, y + 8);
    } 

    if array[x as usize][y as usize] == Square::T && mapping[x as usize][y as usize] == -2 {
        return 1 + count_reached_goals(array, mapping, x, y - 1);
    } else {
        return count_reached_goals(array, mapping, x, y - 1);
    }
}

fn evolve_tribbles(array: [[Square; 8]; 8], array_evo: &mut [[Square; 8]; 8], mapping: [[i8; 8]; 8], x: i8, y: i8) -> [[Square; 8]; 8] {
    if x < 0 {
        return [[Square::U; 8]; 8];
    } 

    if y < 0 {
        return evolve_tribbles(array, array_evo, mapping, x - 1, y + 8);
    }

    *array_evo = set_neighbors(array, array_evo, mapping, x, y);

    evolve_tribbles(array, array_evo, mapping, x, y - 1);

    *array_evo
}

fn set_neighbors(array: [[Square; 8]; 8], result: &mut [[Square; 8]; 8], mapping: [[i8; 8]; 8], x: i8, y: i8) -> [[Square; 8]; 8] {
    let x = x as usize;
    let y = y as usize;

    if (mapping[x][y] == 1 || mapping[x][y] == -2) && array[x][y] == Square::T {
        result[x][y] = Square::T;

        if x as i8 - 1 >= 0 {
            if array[x - 1][y] != Square::U { 
                result[x - 1][y] = Square::T;
            }
        }

        if x + 1 <= 7 {
            if array[x + 1][y] != Square::U { 
                result[x + 1][y] = Square::T;
            }
        }

        if y as i8 - 1 >= 0 {
            if array[x][y - 1] != Square::U { 
                result[x][y - 1] = Square::T;
            }
        }

        if y + 1 <= 7 {
            if array[x][y + 1] != Square::U {
                result[x][y + 1] = Square::T;
            }
        }
    }

    return *result;
}

fn create_mapping(array: [[Square; 8]; 8], x: i8, y: i8) -> [[i8; 8]; 8] {
    if x < 0 {
        return [[0; 8]; 8];
    } else {
        if y < 0 {
            return create_mapping(array, x - 1, y + 8);
        } else {
            let mut mapping = create_mapping(array, x, y - 1);

            let x = x as usize;
            let y = y as usize;

            if array[x][y] == Square::W {
                mapping[x][y] = 3;
            }

            if array[x][y] == Square::V {
                mapping[x][y] = 1;
            }

            if array[x][y] == Square::U {
                mapping[x][y] = -1;
            }

            if array[x][y] == Square::Z {
                mapping[x][y] = -2;
            }

            return mapping;
        }
    }
}

fn cell_is_surrounded(array: &mut [[Square; 8]; 8], x: usize, y: usize) -> bool {
    if x >= 8 || y >= 8 || array[x][y] == Square::U {
        return true; 
    }

    if array[x][y] == Square::T {
        return false;
    }

    if array[x][y] == Square::V || array[x][y] == Square::Z || array[x][y] == Square::W {
        array[x][y] = Square::U;
    } else {
        return true; 
    }

    let mut surrounded = true;

    if x > 0 {
        surrounded &= cell_is_surrounded(array, x - 1, y);
    }

    if y < 7 {
        surrounded &= cell_is_surrounded(array, x, y + 1);
    }

    if x < 7 {
        surrounded &= cell_is_surrounded(array, x + 1, y);
    }

    if y > 0 {
        surrounded &= cell_is_surrounded(array, x, y - 1);
    }

    surrounded
}

fn spawn_tribbles_on_map(mut array: [[Square; 8]; 8], start: &[Option<Coord>; 10], idx: i8) -> [[Square; 8]; 8] {
    if idx < 0 {
        return array;
    } else {
        if let Some(coord) = &start[idx as usize] {
            array[coord.row][coord.col] = Square::T;
        }

        return spawn_tribbles_on_map(array, start, idx - 1)
    }
}
