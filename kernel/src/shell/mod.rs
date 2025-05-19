use crate::{
    framebuffer::ConsoleColor,
    framebuffer::global_writer::{clear_screen, print_fmt},
    framebuffer::color::ColoredWriting,
    print,
    println
};
use alloc::{
    format,
    string::String,
    vec::Vec
};
use spin::{Lazy, Mutex};

pub struct Shell {
    buffer: String,
    prompt: String,
    name: String,
    pub len: usize,
    err: u8,
    command: String,
    args: Vec<String>,
}

impl Shell {
    fn new() -> Self {
        let prompt = String::from("WHAT IS YOUR NAME? ");
        Self {
            buffer: String::new(),
            prompt: prompt.clone(),
            name: String::new(),
            len: (prompt.chars().count() + 1) * 10,
            err: 0,
            command: String::new(),
            args: Vec::new(),
        }
    }

    pub fn append(&mut self, input: char) {
        self.buffer.push(input);
    }

    pub fn init(&mut self) {
        match self.err {
            0 => print!("{}", self.prompt.fg(ConsoleColor::BrightWhite)),
            1 => print!("{}", self.prompt.fg(ConsoleColor::Red)),
            2 => print!("{}", self.prompt.fg(ConsoleColor::Yellow)),
            _ => print!("{}", self.prompt.fg(ConsoleColor::BrightWhite)),
        }
    }

    fn parse_command(&mut self) {
        let parts: Vec<String> = self.buffer.trim().split_whitespace().map(String::from).collect();
        if !parts.is_empty() {
            self.command = parts[0].clone();
            self.args = parts[1..].to_vec();
        } else {
            self.command.clear();
            self.args.clear();
        }
    }

    pub fn exec(&mut self) {
        self.parse_command();

        if self.prompt == "WHAT IS YOUR NAME? " {
            println!("Hello, {}!", self.buffer);
            self.prompt = format!("{}@PortfoliOS -> # ", self.buffer);
            self.name = self.buffer.clone();
            self.buffer.clear();
            self.len = (self.prompt.chars().count() + 1) * 10;
        } else {
            self.err = 0;

            match self.command.as_str() {
                "whoami" => {
                    println!("\
                    Hello there {}! I am Agamjot Singh Bindra, a student of 11th grade, at Bal Bharati Public School, and the creator of \
                PortfoliOS, and it's shell, AgamShell(shortened to ASH). ", self.name);
                },
                "projects" => {let print = "
I have worked on the following projects:
1. PortfoliOS - A simple OS, with a shell, and a framebuffer.
2. PortfoliOS-CLI - A simple CLI, for my portfolio (indev)
3. CareerCompass - A Career Guidance website, made using react, nodejs, and firebase.
4. AI-Snake-Game - A simple snake game, with AI, made using python.(I am not good at naming things...)
";
                    println!("{}" ,print);},
                "clear" => {
                    clear_screen();
                },
                "whatilike" => {
                    let print = "
I like the following things:
1. Coding - I love coding, and I am learning new things every day.
2. Gaming - I like to play games, such as Kerbal Space Program, even though with school, and all, I don't really have the time to do so.
3. 8TXt745lcHnuFVncMB3em5enK0ex63Sa \x1b[31m(ERROR: MEMORY_OVERFLOW... TERMINATING USER)\x1b[0m
";
                    println!("{}" ,print);
                    for _ in 0..5e6 as u128 {
                        core::hint::spin_loop();
                    }
                    println!("That was a joke... I am not a hacker... Promise...");
                    println!("3. was supposed to be kernel debugging...");
                    for _ in 0..5e5 as u128 {
                        core::hint::spin_loop();
                    }
                    let print = "                 ___-----------___
           __--~~                 ~~--__
       _-~~                             ~~-_
    _-~                                     ~-_
   /                                           \\
  |                                             |
 |                                               |
 |                                               |
|                                                 |
|                                                 |
|                                                 |
 |                                               |
 |  |    _-------_               _-------_    |  |
 |  |  /~         ~\\           /~         ~\\  |  |
  ||  |             |         |             |  ||
  || |               |       |               | ||
  || |              |         |              | ||
  |   \\_           /           \\           _/   |
 |      ~~--_____-~    /~V~\\    ~-_____--~~      |
 |                    |     |                    |
|                    |       |                    |
|                    |  /^\\  |                    |
 |                    ~~   ~~                    |
  \\_         _                       _         _/
    ~--____-~ ~\\                   /~ ~-____--~
         \\     /\\                 /\\     /
          \\    | ( ,           , ) |    /
           |   | (~(__(  |  )__)~) |   |
            |   \\/ (  (~~|~~)  ) \\/   |
             |   |  [ [  |  ] ]  /   |
              |                     |
               \\                   /
                ~-_             _-~
                   ~--___-___--~".fg(ConsoleColor::Red);
                    println!("{}" ,print);
                },
                "help" => {
                    println!("Available commands: whoami, projects, whatilike, clear, help, echo, shutdown, exit, portfoliofetch");
                    println!("Try running ls...")
                },
                "echo" => self.err = self.handle_echo(),
                "portfoliofetch" => {
                    println!("{}" ,"  _____           _    __      _ _  ____   _____ 
 |  __ \\         | |  / _|    | (_)/ __ \\ / ____|
 | |__) |__  _ __| |_| |_ ___ | |_| |  | | (___  
 |  ___/ _ \\| '__| __|  _/ _ \\| | | |  | |\\___ \\ 
 | |  | (_) | |  | |_| || (_) | | | |__| |____) |
 |_|   \\___/|_|   \\__|_| \\___/|_|_|\\____/|_____/".fg(ConsoleColor::BrightRed));
                        println!("Uptime: idk... don't you have a clock?");
                        println!("Kernel Version: 0.0.1");
                        println!("PortfoliOS Version: 0.0.1");
                        println!("Developer: Agamjot Singh Bindra");
                        println!("Website: ummm... good question...");
                }


                "exit" | "execute66" => {
                    let format = "War! The Republic is crumbling under attacks by the ruthless Sith Lord, Count Dooku. There are heroes on both sides. Evil is everywhere.
In a stunning move, the fiendish droid leader, General Grievous, has swept into the Republic capital and kidnapped Chancellor Palpatine, leader of the Galactic Senate.
As the Separatist Droid Army attempts to flee the besieged capital with their valuable hostage, two Jedi Knights lead a desperate mission to rescue the captive Chancellor...\n";


                for char in format.chars(){
                    print_fmt(format_args!("{}", char), ConsoleColor::BrightYellow, ConsoleColor::Black);
                    for _ in 0..100_000 {
                        core::hint::spin_loop();
                    }
                }
                println!("(c) Whoever made star wars episode 3...");
                }
                "rename" => self.err = self.handle_rename(),
                "bye" => {
                    println!("Bye {}! See you later!", self.name);
                    println!("Exiting...");
                    for _ in 0..5e6 as u128 {
                        core::hint::spin_loop();
                    }
                    println!("Exiting... (c) Agamjot Singh Bindra");
                    println!("Bye!");
                }
                "calc" => self.err = self.handle_calc(),
                _ => {
                    if ["ls", "touch", "cd", "mkdir", "cat"].contains(&self.command.as_str()) {
                        println!("Bro... there is no filesystem... <add skull emoji here when emojis are supported... //todo>");
                        self.err = 2;
                    } else {
                        self.err = 1;
                        println!("{} is not a valid command", self.buffer);
                    }
                }
            }
        }
        self.buffer.clear();
        self.init();
    }

    fn handle_rename(&mut self) -> u8 {
        if self.args.is_empty() {
            println!("Usage: rename <old_name> <new_name>");
            return 1;
        }
        if self.args.len() != 2 {
            println!("Usage: rename <old_name> <new_name>");
            return 1;
        }
        let old_name = &self.args[0];
        let new_name = &self.args[1];
        println!("Renaming {} to {}", old_name, new_name);
        if self.name == *old_name {
            self.name = new_name.clone();
            println!("Renamed {} to {}", old_name, new_name);
        } else {
            println!("{} is not your name", old_name);
        }
        0
    }

    fn handle_calc(&self) -> u8{
        if self.args.is_empty(){
            println!("Usage: calc <num1> <operator> <num2>");
            println!("Operators: +, -, *, /");
            return 1;
        }
        if self.args.len() != 3 {
            println!("Usage: calc <num1> <operator> <num2>");
            return 1;
        }
        let num1: f64 = self.args[0].parse().unwrap_or_else(
            |_| {
                println!("Invalid number: {}", self.args[0]);
                0.0
            }
        );
        let num2: f64 = self.args[2].parse().unwrap_or_else(
            |_| {
                println!("Invalid number: {}", self.args[2]);
                0.0
            }
        );

        match self.args[1].as_str() {
            "+" => println!("{} + {} = {}", num1, num2, num1 + num2),
            "-" => println!("{} - {} = {}", num1, num2, num1 - num2),
            "*" => println!("{} * {} = {}", num1, num2, num1 * num2),
            "/" => {
                if num2 == 0.0 {
                    println!("Division by zero is not allowed");
                } else {
                    println!("{} / {} = {}", num1, num2, num1 / num2);
                }
            },
            _ => {
                println!("Invalid operator: {}", self.args[1]);
            }

        };
        0
    }

    fn handle_echo(&self) -> u8 {
        if self.args.is_empty() {
            println!("Usage: echo [-c <color>] <message>");
            return 1;
        }

        // Check for the color switch: echo -c <color> <message>
        if self.args[0] == "-c" && self.args.len() > 2 {
            let color_input = self.args[1].to_lowercase();
            let color = match color_input.as_str() {
                "black" => ConsoleColor::Black,
                "red" => ConsoleColor::Red,
                "green" => ConsoleColor::Green,
                "yellow" => ConsoleColor::Yellow,
                "blue" => ConsoleColor::Blue,
                "magenta" => ConsoleColor::Magenta,
                "cyan" => ConsoleColor::Cyan,
                "white" => ConsoleColor::White,
                "brightblack" => ConsoleColor::BrightBlack,
                "brightred" => ConsoleColor::BrightRed,
                "brightgreen" => ConsoleColor::BrightGreen,
                "brightyellow" => ConsoleColor::BrightYellow,
                "brightblue" => ConsoleColor::BrightBlue,
                "brightmagenta" => ConsoleColor::BrightMagenta,
                "brightcyan" => ConsoleColor::BrightCyan,
                "brightwhite" => ConsoleColor::BrightWhite,
                _ => {
                    println!("Invalid color. Supported: black, red, green, yellow, blue, magenta, cyan, white, brightblack, brightred, brightgreen, brightyellow, brightblue, brightmagenta, brightcyan, brightwhite.");
                    return 1;
                }
            };
            let message = self.args[2..].join(" ");
            println!("{}", message.fg(color));
        } else {
            let message = self.args.join(" ");
            println!("{}", message);
        }
        0
    }

    pub fn pop(&mut self) {
        if !self.buffer.is_empty() {
            self.buffer.pop();
        }
    }
}


pub static GLOBAL_SHELL: Lazy<Mutex<Shell>> = Lazy::new(|| {
    Mutex::new(Shell::new())
});
