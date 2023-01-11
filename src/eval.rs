use crate::token::{Token, POSSIBLETOKENVALUE, TOKENS};

// supports operations on integers with the length of 1
pub fn eval_tokens(t: Vec<Token>){
    let mut res: f64 = 0.0;

    if let POSSIBLETOKENVALUE::INTEGER(i) = t.get(0).unwrap().tvalue {
        res = i as f64;
    }

    for i in 1..(t.len()-1) {
        let tok = t.get(i).unwrap();
        let mut nval = 0.0;

        if let POSSIBLETOKENVALUE::INTEGER(v) = t.get(i+1).unwrap().tvalue {
            nval = v as f64;
        }

        match tok.ttype {
            TOKENS::PLUS => res += nval,
            TOKENS::MINUS => res -= nval,
            TOKENS::ASTERISK => res *= nval,
            TOKENS::SLASH => res /= nval,
            TOKENS::MODULO => res = res % nval,
           _ => (), 
        };
    }
    println!("{}", res);
}
