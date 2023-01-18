extern crate termion;
extern crate termios;

#[macro_use]
extern crate lazy_static;

use termion::{color, cursor, clear};
use std::io;
use std::io::Read;
use std::io::Write;
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use std::{thread, time};
use std::sync::RwLock;
use std::sync::Arc;


const WIDTH: u16 = 20;
const HEIGHT: u16 = 20;

const WIDTH_OFFSET: u16 = 59;
const HEIGHT_OFFSET: u16 = 10;

#[derive(Clone)]
enum Direction {

    Left,
    Right,
    Up,
    Down,
}

struct Snake {

    body: Vec<u16>,
    direction: Direction,
}

lazy_static! {
    static ref key_buffer: Arc<RwLock<[u8; 1]>> = Arc::new(RwLock::new([0]));
}


fn get_x_from_coord(coords: u16) -> u16 {
    coords % WIDTH
}

fn get_y_from_coord(coords: u16) -> u16 {
    coords / HEIGHT
}

fn move_head(snake: &mut Snake, direction: Direction) {
    match direction {   
        Direction::Up => snake.body[0] -= HEIGHT,
        Direction::Down => snake.body[0] += HEIGHT,
        Direction::Left => snake.body[0] -= 1,
        Direction::Right => snake.body[0] += 1,
    }
}

fn draw_board() {
    
    for i in 0..WIDTH { print!("{color}{goto}-", color = color::Fg(color::White), goto = cursor::Goto(WIDTH_OFFSET + i, HEIGHT_OFFSET)) ; }

    println!(" ");

    for i in 1..HEIGHT {
        println!("{color}{goto}|", 
            color = color::Fg(color::White), 
            goto = cursor::Goto(WIDTH_OFFSET, HEIGHT_OFFSET + i)); 


        println!("{color}{goto}|", 
            color = color::Fg(color::White), 
            goto = cursor::Goto(WIDTH_OFFSET + WIDTH, HEIGHT_OFFSET + i)); 
    }

    for i in 0..WIDTH { 
        print!("{color}{goto}-", 
            color = color::Fg(color::White), 
            goto = cursor::Goto(WIDTH_OFFSET + i, HEIGHT_OFFSET + HEIGHT)) ; 
    }
}

fn update_snake(snake: &mut Snake, direction: Direction) {

    // Update tail
    for i in (1..snake.body.len()).rev() { snake.body[i] = snake.body[i - 1]; }
    
    // Update head
    move_head(snake, direction);
}

fn print_snake(snake: &Snake) {

    for i in 0..snake.body.len() { 
        println!("{color}{goto}*",
            color = color::Fg(color::Red), 
            goto = cursor::Goto(get_x_from_coord(snake.body[i]) + WIDTH_OFFSET + 1, get_y_from_coord(snake.body[i]) + HEIGHT_OFFSET + 1)); 
    }
}

fn clear_snake(snake: &Snake) {

    for i in 0..snake.body.len() { 
        println!("{goto} ",
            goto = cursor::Goto(get_x_from_coord(snake.body[i]) + WIDTH_OFFSET + 1, get_y_from_coord(snake.body[i]) + HEIGHT_OFFSET + 1)); 
    }

}

fn user_input(snake: &mut Snake, prev_direction: Direction) {

    thread::spawn(move || {

        let stdin = 0;
        let termios = Termios::from_fd(stdin).unwrap();
        let mut new_termios = termios.clone();   
        new_termios.c_lflag &= !(ICANON | ECHO); 
        tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
        let stdout = io::stdout();
        let mut reader = io::stdin();
        stdout.lock().flush().unwrap();

        reader.read_exact(&mut key_buffer).unwrap();
        println!("You have hit: {key_buffer}");
        tcsetattr(stdin, TCSANOW, & termios).unwrap();
    });
}


fn main() {
    
    
    println!("{clear}", clear = clear::All);
    let mut snake = Snake{ body: Vec::new(), direction: Direction::Down };
    snake.body.push(10);
    snake.body.push(11);
    snake.body.push(12);
    loop {

        draw_board();

        clear_snake(&snake);
        let mut direction = snake.direction.clone();
        user_input(&mut snake, direction);
        print_snake(&snake);

        thread::sleep(time::Duration::from_millis(100));
    }

   
    //println!("{clear}YOU LOST!!! noob", clear = clear::All);
    
      
}
