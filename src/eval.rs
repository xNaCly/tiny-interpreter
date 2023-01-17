use crate::token::{Token, POSSIBLETOKENVALUE, TOKENS};

// supports operations on integers with the length of 1
pub fn eval_tokens(t: Vec<Token>){
    let mut res: f64 = 0.0;

    for i in 1..(t.len()-1) {
        let tok = t.get(i).unwrap();
        let mut cval = 0.0;

        if let POSSIBLETOKENVALUE::INTEGER(v) = t.get(i+1).unwrap().tvalue {
            cval = v as f64;
        }

        match tok.ttype {
            TOKENS::PLUS => res += cval,
            TOKENS::MINUS => res -= cval,
            TOKENS::ASTERISK => res *= cval,
            TOKENS::SLASH => res /= cval,
            TOKENS::MODULO => res = res % cval,
           _ => (), 
        };
    }

    println!("{}", res);
}

#[cfg(test)]
mod eval_tokens_tests {
    use crate::scanner::scan;

    #[test]
    fn adding_integers_of_length_1(){
        let mut inp = "1+2";
        let mut tokens = scan(inp);
        assert_eq!(super::eval_artithmetic(tokens), 3.0);

        inp = "2+1";
        tokens = scan(inp);
        assert_eq!(super::eval_artithmetic(tokens), 3.0);

        inp = "0+0";
        tokens = scan(inp);
        assert_eq!(super::eval_artithmetic(tokens), 0.0);
        
        inp = "9+9";
        tokens = scan(inp);
        assert_eq!(super::eval_artithmetic(tokens), 18.0);
    }

    #[test]
    fn subtracting_integers_of_length_1(){
        let mut inp = "1-2";
        let mut tokens = scan(inp);
        assert_eq!(super::eval_artithmetic(tokens), -1.0);

        inp = "2-1";
        tokens = scan(inp);
        assert_eq!(super::eval_artithmetic(tokens), 1.0);

        inp = "0-0";
        tokens = scan(inp);
        assert_eq!(super::eval_artithmetic(tokens), 0.0);
        
        inp = "9-9";
        tokens = scan(inp);
        assert_eq!(super::eval_artithmetic(tokens), 0.0);

        inp = "0-9";
        tokens = scan(inp);
        assert_eq!(super::eval_artithmetic(tokens), -9.0);
    }


    #[test]
    fn multiplying_integers_of_length_1(){
        let mut inp = "1*2";
        let mut tokens = scan(inp);
        assert_eq!(super::eval_artithmetic(tokens), 2.0);

        inp = "2*1";
        tokens = scan(inp);
        assert_eq!(super::eval_artithmetic(tokens), 2.0);

        inp = "0*0";
        tokens = scan(inp);
        assert_eq!(super::eval_artithmetic(tokens), 0.0);
        
        inp = "9*9";
        tokens = scan(inp);
        assert_eq!(super::eval_artithmetic(tokens), 81.0);
    }

    #[test]
    fn dividing_integers_of_length_1(){
        let mut inp = "1/2";
        let mut tokens = scan(inp);
        assert_eq!(super::eval_artithmetic(tokens), 0.5);

        inp = "2/1";
        tokens = scan(inp);
        assert_eq!(super::eval_artithmetic(tokens), 2.0);

        inp = "9/9";
        tokens = scan(inp);
        assert_eq!(super::eval_artithmetic(tokens), 1.0);
    }

    #[test]
    fn modulo_integers_of_length_1(){
        let mut inp = "1%2";
        let mut tokens = scan(inp);
        assert_eq!(super::eval_artithmetic(tokens), 1.0);

        inp = "2%3";
        tokens = scan(inp);
        assert_eq!(super::eval_artithmetic(tokens), 2.0);

        inp = "9%9";
        tokens = scan(inp);
        assert_eq!(super::eval_artithmetic(tokens), 0.0);
    }
}

