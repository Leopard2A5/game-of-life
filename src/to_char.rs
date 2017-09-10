use game_of_life::Cell;

pub trait ToChar {
	fn to_char(&self) -> char;
}

impl ToChar for Cell {
	fn to_char(&self) -> char {
		match *self {
			Cell::Dead => ' ',
			Cell::Live => 'X',
		}
	}
}
