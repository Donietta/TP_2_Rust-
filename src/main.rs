use santiago::lexer::LexerRules;

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

fn main() {
    let lexer_rules = lexer_rules();

    let input = "forward 100 right 90";

    let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();

    println!("{:#?}", lexemes);
}
