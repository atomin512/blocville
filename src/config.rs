pub const RESIDENTIAL_POLLUTION_PER_TICK: i8 = 1;
pub const EMPTY_POLLUTION_PER_TICK: i8 = 0;

// 8 neighbors * 2(we could afford give half of value)
// 16 is vale that divides half of the value for 8 neighbors(32 / 16 = 2 or 32(value) / 8(neighbors) / 2(half) = 2)
// TODO(???): make it 8 + 1 so treshold will be 9 and less notable in game
// OR
// TODO(???): move game to f32 but it's pretty expensive
// OR
// Do nothing with it, its works simple
pub const POLLUTION_SPREAD_THRESHOLD: u8 = 9;

pub const OFFSETS_OF_8: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];