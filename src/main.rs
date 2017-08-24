extern crate lib_game_of_life;
extern crate termion;

mod to_char;

use lib_game_of_life::GameOfLife;
use std::thread;
use std::time;
use to_char::ToChar;

fn main() {
    let mut gof = GameOfLife::new(30, 30);

	gof.awake(14, 15);
	gof.awake(15, 15);
	gof.awake(16, 15);

	loop {
		clear_screen();
		print_gof(&gof);
		gof.step();
		thread::sleep(time::Duration::from_millis(500));
	}
}

fn print_gof(gof: &GameOfLife) {
	for _ in 0..gof.width() + 2 {
		print!("-");
	}
	println!();

	for y in 0..gof.height() {
		print!("|");
		for x in 0..gof.width() {
			print!("{}", gof.get(x, y).to_char());
		}
		println!("|");
	}

	for _ in 0..gof.width() + 2 {
		print!("-");
	}
	println!();
}

fn clear_screen() {
	print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
}
