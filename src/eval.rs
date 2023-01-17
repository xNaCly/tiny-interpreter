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
