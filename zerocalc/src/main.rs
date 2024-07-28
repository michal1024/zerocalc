use engine::Calculator;
use engine::Parser;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

const HELLO: &str = "Welcome to Zerocalc!
Type \"help\" to print help message.
Type \"exit\" or press ctrl-d to exit.
";

const HELP: &str = "
** Editor navigation **

    - left or right arrow to edit the input string
    - up or down arrow to navigate history
    - delete, backspace to delete characters
    - type 'exit' or press ctrl-d to exit
    - type 'help' to see this message

** Numbers **

    Numbers can have spaces and underscores:

    1 000_000
    3.14 15
    . 123

    Floats can use scientific notation:

    2e-2 == 0.02

** Operators **

    + addition, 
    - subtraction, 
    * multiplication,
    / division, 
    ^ power, 
    % modulo

** Constants ** 

    e, pi

** Functions **

    - abs(x)                    - absolute value of x
    - sin(x), cos(x), tan(x)    - x in radians
    - asin(x), acos(x), atan(x) - x in radians
    - ln(x), log10(x)           - natural logarithm, common logarith
    - log(x, y)                 - base y logarithm
    - sqrt(x)                   - square root
    - root(x, y)                - y'th root of x

** Variables **

    x = (1 + 2)
    y = x / 3
";

const GOODBYE: &str = "Goodbye!";

struct Repl {
    calc: Calculator,
}

impl Repl {

    fn new() -> Self {
        Repl {
            calc: Calculator::new(),
        }
    }

    fn run(&mut self) {
        println!("{}", HELLO);
        let mut input = DefaultEditor::new().unwrap();
        loop {
            match input.readline(">") {
                Ok(s) if s == "exit" => { println!("{}", GOODBYE); break },
                Ok(s) if s == "help" => println!("{}",HELP),
                Ok(s) =>{
                    self.eval(&s);
                    input.add_history_entry(s).unwrap_or_default();
                }
                Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                    println!("{}", GOODBYE); 
                    break
                }
                Err(err) => {
                    println!("{:?}", err)
                }
            }   
        }
    }

    fn eval(&mut self, buffer: &str) {
        let mut p = Parser::new(buffer);
        match p.parse() {
            Ok(true) => {
                let res = self.calc.eval(&p.program);
                println!("{res}");
            },
            Err(e) => {
                println!("{:spaces$}{:^<marks$}", " ", "^", spaces=e.span.pos + 1, marks=e.span.len);
                println!("{}", e.message);
            },
            _ => ()
        }
    }
    }

fn main() {
    Repl::new().run();
}
