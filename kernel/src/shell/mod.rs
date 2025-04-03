use alloc::format;
use alloc::string::String;
use spin::{Lazy, Mutex};
use crate::{print, println};
use crate::framebuffer::color::ColoredWriting;
use crate::framebuffer::ConsoleColor;
use crate::framebuffer::global_writer::clear_screen;

pub struct Shell {
    buffer: String,
    prompt: String,
    name: String,
    pub len: usize,
    err: u8
}

impl Shell {
    fn new() -> Self {
        Self {
            buffer: String::new(),
            prompt: String::from("WHAT IS YOUR NAME? "),
            name: String::new(),
            len: (String::from("WHAT IS YOUR NAME? ").chars().count()  + 1) * 10,
            err: 0
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

    pub fn exec(&mut self) {
        if self.prompt == "WHAT IS YOUR NAME? " {
            println!("Hello, {}!", self.buffer);
            self.prompt = format!("{}@PortfoliOS -> # ", self.buffer);
            self.name = self.buffer.clone();
            self.buffer.clear();
            self.len = (self.prompt.chars().count() + 1) * 10;
        }
        else {
            if self.buffer == "whoami"{
                println!("Hello there {}! I am Agamjot Singh Bindra, a student of 11th grade, at Bal Bharathi Public School, and the creator of \
                PortfoliOS, and it's shell, AgamShell(shortened to ASH). ", self.name);
            }
            else if self.buffer == "projects" {
                let print = "
I have worked on the following projects:
1. PortfoliOS - A simple OS, with a shell, and a framebuffer.
2. PortfoliOS-CLI - A simple CLI, with a shell, and a framebuffer.
3. CareerCompass - A career Guidance website, made using react, nodejs, and firebase.
4. AI-Snake-Game - A simple snake game, with AI, made using python.(I am not good at naming things...)
";
                println!("{}" ,print);
            }else if self.buffer == "whatilike" {
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
                   ~--___-___--~
                   ".fg(ConsoleColor::Red);
                println!("{}" ,print);
            } else if self.buffer == "clear" {
                println!("Clearing screen...");
                clear_screen()
            } else if self.buffer == "help" {
                println!("Available commands: whoami, projects, whatilike, clear, help");
            } else if self.buffer == "exit" {
                println!("Exiting shell...");
            } else if self.buffer == "reboot" {
                println!("Rebooting system...");
            } else if self.buffer == "shutdown" {
                println!("Shutting down system...");
            }
            else{
                for cmd in ("ls, touch, cd, mkdir, cat").split(", ") {
                    if self.buffer == cmd {
                        println!("You do know, that there is no file system here, and there never will be, right?");
                        self.err = 2;
                    }
                    else { self.err = 1 }
                }
            }
        }
        self.buffer.clear();
        self.init();
    }
    pub fn pop(&mut self) {
        if !self.buffer.is_empty() {
            self.buffer.pop().unwrap();
        }
    }
}


pub static GLOBAL_SHELL: Lazy<Mutex<Shell>> = Lazy::new(|| {
    Mutex::new(Shell::new())
});
