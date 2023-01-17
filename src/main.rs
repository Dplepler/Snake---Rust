extern crate termion;
use termion::{color, cursor, clear};
use termion::input::TermRead;
use std::{thread, time};
use termion::event::Key;

const WIDTH: u16 = 20;
const HEIGHT: u16 = 20;


const WIDTH_OFFSET: u16 = 59;
const HEIGHT_OFFSET: u16 = 10;

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


    let mut stdin = termion::async_stdin();
    let mut it = stdin.keys();
    
    let b = it.next();
    match b {
        Some(x) => match x {
            Ok(k) => {
                match k {
                    Key::Up => update_snake(snake, Direction::Up),
                    Key::Down => update_snake(snake, Direction::Down),
                    Key::Right => update_snake(snake, Direction::Right),
                    Key::Left => update_snake(snake, Direction::Left),
                    _ => update_snake(snake, prev_direction),
                }
            },
            _ => {}
    },
    None => {}
    }
}


fn main() {
    
    
    println!("{clear}", clear = clear::All);
    let mut snake = Snake{ body: Vec::new(), direction: Direction::Up };
    snake.body.push(10);
    snake.body.push(11);
    snake.body.push(12);
    loop {

        draw_board();

        clear_snake(&snake);
        
        let direction = snake.direction;
        user_input(&mut snake, direction);
        print_snake(&snake);

        thread::sleep(time::Duration::from_millis(100));
    }

   
    //println!("{clear}YOU LOST!!! noob", clear = clear::All);
    
      
}
