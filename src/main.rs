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

struct Logo {
    position_x: f32,
    position_y: f32,
    orientation : f32, // en degrés
    pen_status : bool, // on considère ici que le booléen est vrai quand le stylo est baissé
    svg_contenu : String, // String dans laquelle on sauvegarde le contenu du fichier SVG au cours du programme Logo
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
impl Logo {

    fn forward(&mut self, distance: f32) {
        let rad = self.orientation.to_radians();

        let new_x = self.position_x + distance * rad.cos();
        let new_y = self.position_y + distance * rad.sin();

        if self.pen_status {
            self.svg_contenu.push_str(&format!(
                r#"<path d="M {} {} L {} {}" stroke="black"/>"#,
                self.position_x, self.position_y, new_x, new_y
            ));
        }

        self.position_x = new_x;
        self.position_y = new_y;
    }

    fn left(&mut self, orientation: f32) {
        self.orientation -= orientation;
    }

    fn right(&mut self, orientation: f32) {
        self.orientation += orientation;
    }

    fn backward(&mut self, distance: f32) {
        self.forward(-distance);
    }
    fn compile(&mut self, ast: AST) {

        match ast {
            AST::Program(commands) => {
                for cmd in commands {
                    self.compile(cmd);
                }
            }

            AST::Command(order, value) => {
                match order {
                    Order::Forward => self.forward(value as f32),
                    Order::Backward => self.forward(-(value as f32)),
                    Order::Left => self.left(value as f32),
                    Order::Right => self.right(value as f32),
                }
            }

            AST::Empty => {}
            _ => {}
        }
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

    // création d'une instance de Logo pour tester
    let mut logo = Logo {
        position_x: 0.0,
        position_y: 0.0,
        orientation: 0.0,
        pen_status: true,
        svg_contenu: String::new(),
    };

    // appel de compile
    logo.compile(ast);

    // wrap svg
    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" width="300" height="300">
{}
</svg>"#,
        logo.svg_contenu
    );

    //sauvegarde
    std::fs::write("output.svg", svg).expect("Unable to write SVG file");
    println!("SVG generated: output.svg");
}
