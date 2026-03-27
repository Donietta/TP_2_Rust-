use santiago::lexer::LexerRules;
use santiago::grammar::Grammar;

#[derive(Debug)]
pub enum AST {
    Program(Vec<AST>),              // liste de commandes et fin
    Command(Order, i32),              // wrapper 
    Order(Order),
    Number(i32),                    // <number>
    Empty,                          // règle ""
    ProgramEnd,                     // fin de programme (""),
}

#[derive(Debug)]
pub enum Order {
    Forward,
    Backward,
    Left,
    Right,
}

fn grammar() -> Grammar<AST> {
    santiago::grammar!(

    //Program
    "program" => rules "command" "program"
        => |mut nodes| {
            let mut commands = Vec::new();
            commands.push(nodes.remove(0));
            if let AST::Program(rest) = nodes.remove(0){
                commands.extend(rest);
            }
            AST::Program(commands)
        };

    "program" => empty
        => |_| AST::Empty;

    //Commande = ordre + nombre

    "command" => rules "order" "number"
        => |mut nodes|{
            let order = nodes.remove(0);
            let number = nodes.remove(0);

            if let (AST::Order(o),AST::Number(n))=(order,number){
                AST::Command(o,n)
            } 
            else {
                panic!("invalid command structure");
            }
        };


    //Ordres possibles
    "order" => lexemes "FORWARD"
        => |_| AST::Order(Order::Forward);
    "order" => lexemes "BACKWARD"
        => |_| AST::Order(Order::Backward);
    "order" => lexemes "LEFT"
        => |_| AST::Order(Order::Left);
    "order" => lexemes "RIGHT"
        => |_| AST::Order(Order::Right);

    //Nombres

    "number" => lexemes "INT"
       => |lexemes| {
                let lex = &lexemes[0];
                let value = lex.raw.parse::<i32>().unwrap();
                AST::Number(value)
            };
    )
}

pub fn lexer_rules() -> LexerRules{
    santiago::lexer_rules!(
        //Ignorer les espaces
        "DEFAULT" | "WS" = pattern r"\s+" => |lexer| lexer.skip();

        //Commandes
        "DEFAULT" | "FORWARD"  = string "forward";
        "DEFAULT" | "BACKWARD" = string "backward";
        "DEFAULT" | "LEFT"     = string "left";
        "DEFAULT" | "RIGHT"    = string "right";

        //Nombres
        // un ou plusieurs chiffres de 0 à 9 seront mappés en "INT"
        "DEFAULT" | "INT" = pattern r"[0-9]+";

    )
    
}   

fn eval(ast : &AST){
    match ast {
        AST::Program(commands) => {
            for cmd in commands{
                eval(cmd);
            }
            println!("Stop");
        }

        AST::Command(order,value) => {
            match order{
                Order::Forward => println!("Avance de {} unités", value),
                Order::Backward => println!("Recule de {} unités", value),
                Order::Left => println!("Tourne à gauche de {} degrés", value),
                Order::Right => println!("Tourne à droite de {} degrés", value),
            }
        }

        AST::Empty | AST::ProgramEnd => {
            //ne fait rien
        }

        _ => {}
    }
}
fn main() {
    let lexer_rules = lexer_rules();
    let grammar = grammar();

    let input = "forward 100 right 90";

    let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();

    let parse_trees = santiago::parser::parse(&grammar,&lexemes)
        .expect("syntax error");

    
    let ast = parse_trees[0].as_abstract_syntax_tree();

    println!("{:?}", ast);

    eval(&ast);
}
