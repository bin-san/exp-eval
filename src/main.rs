/*
exp-eval
An Expression Evaluator program that can be used as a backened of scientific calculators
Author: Sandipan Chowdhury
Date: Dec 2022
Step by step guide to build everything from scratch
*/
/*
    Chapter1: The Data structure
    We start by writing a data structure called Node. This is kind
    of a tree data structure. A Node consists of three members
    An operator and two operand.
    Operator enum contains name of the basic operation.
    Operand enum contains Expression and Number. Both wraps around
    the smart pointer. The smart pointer points to either a Node or
    a floating point.
    Then we implement the calculate method to the Node. This menthod
    do the specified operation between the two operand if both of them
    are float. If not, the first calculate the child node to get a float
    value then do the specified operation.
*/
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}
enum Operand {
    Expression(Box<Node>),
    Number(Box<f64>),
}
struct Node {
    operator: Operator,
    operand1: Operand,
    operand2: Operand,
}
impl Node {
    fn calculate(&self) -> f64 {
        let (mut x, mut y) = (0.0, 0.0);
        match &self.operand1 {
            Operand::Expression(some) => {
                x = some.calculate();
            }
            Operand::Number(some) => {
                x = **some;
            }
        }
        match &self.operand2 {
            Operand::Expression(some) => {
                y = some.calculate();
            }
            Operand::Number(some) => {
                y = **some;
            }
        }
        match self.operator {
            Operator::Add => x + y,
            Operator::Sub => x - y,
            Operator::Mul => x * y,
            Operator::Div => x / y,
        }
    }
}
/*
    Chapter2: Preprocessor
    In this part we will write a preprocessor that processes the string
    input and gives error if appropriate syntax is not followed.
    Steps:
    1. Remove all the whitespace in the string input
    2. Check if the string contains any illegal characters
    3. Check illegal use of braces
    4. Check syntax
    5. Explicitly correct the implicit zero before + and - sign
*/
struct SyntaxError {
    index: usize,
    message: String,
}
fn preprocess(s: &String) -> Result<String, SyntaxError> {
    let mut primitive = s.to_string(); //made a copy to process further
    primitive = primitive.trim().replace(" ", ""); //removing all the white spaces
                                                   //all the error references are according to the primitive and not the real string input
                                                   //notice primitive has no whitespace character
    let legal_notation = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "(", ")", "*", "/", "+", "-", ".",
    ];
    let mut pp1 = primitive.clone();
    for i in legal_notation {
        pp1 = pp1.replace(i, "");
    }
    //the pp1 must be an empty string if there is no illegal notation
    if pp1 != "" {
        return Err(SyntaxError {
            index: primitive.find(&pp1[0..1]).unwrap(),
            message: String::from("Illegal character."),
        });
    }
    //Check illegal use of braces
    let (mut opb, mut clb) = (0, 0);
    let mut objg = false; //opening braces just got
    let mut no_rh_oprnd_minus_pos = vec![];
    for (index, i) in primitive.chars().enumerate() {
        match i {
            '(' => {
                opb += 1;
                objg = true;
            }
            ')' => {
                clb += 1;
                if clb > opb {
                    return Err(SyntaxError {
                        index: index,
                        message: String::from("Improper use of braces."),
                    });
                }
                objg = false;
            }
            '-' | '+' => {
                if index == 0 {
                    no_rh_oprnd_minus_pos.push(index);
                }
                if objg {
                    no_rh_oprnd_minus_pos.push(index);
                }
                objg = false;
            }
            _ => {
                objg = false;
            }
        }
    }
    if opb != clb {
        return Err(SyntaxError {
            index: primitive.len() - 1,
            message: String::from("Braces are not closed properly."),
        });
    }

    //Checking and determining illegal use of syntax
    let mut result = primitive;
    #[derive(PartialEq)]
    enum Symbol {
        add,
        mul,
        div,
        sub,
        opbr,
        clbr,
        dig,
        dec,
        non,
    }
    use Symbol::*;
    let (mut prev_sym, mut curr_sym) = (non, non);
    let result_length = result.len();
    for (index, i) in result.chars().enumerate() {
        if index == 0 {
            match i {
                '+' => curr_sym = add,
                '-' => curr_sym = sub,
                '*' => curr_sym = mul,
                '/' => curr_sym = div,
                '.' => curr_sym = dec,
                '0'..='9' => curr_sym = dig,
                '(' => curr_sym = opbr,
                ')' => curr_sym = clbr,
                _ => {
                    return Err(SyntaxError {
                        index: 0,
                        message: String::from("Illegal character."),
                    });
                }
            }
            if curr_sym == mul || curr_sym == div {
                return Err(SyntaxError {
                    index: 0,
                    message: String::from("1st operand is missing."),
                });
            }
            continue;
        }
        match i {
            '+' => {
                prev_sym = curr_sym;
                curr_sym = add;
                if vec![add, mul, div, sub].contains(&prev_sym) {
                    return Err(SyntaxError {
                        index: index,
                        message: String::from("Two operators must have an operand between them"),
                    });
                }
                if prev_sym == dec {
                    return Err(SyntaxError {
                        index: index,
                        message: String::from("A number can't end with a decimal point."),
                    });
                }
            }
            '-' => {
                prev_sym = curr_sym;
                curr_sym = sub;
                if vec![add, mul, div, sub].contains(&prev_sym) {
                    return Err(SyntaxError {
                        index: index,
                        message: String::from("Two operators must have an operand between them"),
                    });
                }
                if prev_sym == dec {
                    return Err(SyntaxError {
                        index: index,
                        message: String::from("A number can't end with a decimal point."),
                    });
                }
            }
            '*' => {
                prev_sym = curr_sym;
                curr_sym = mul;
                if vec![add, mul, div, sub].contains(&prev_sym) {
                    return Err(SyntaxError {
                        index: index,
                        message: String::from("Two operators must have an operand between them"),
                    });
                }
                if prev_sym == dec {
                    return Err(SyntaxError {
                        index: index,
                        message: String::from("A number can't end with a decimal point."),
                    });
                }
                if prev_sym == opbr {
                    return Err(SyntaxError {
                        index: index,
                        message: String::from("First operand is missing."),
                    });
                }
            }
            '/' => {
                prev_sym = curr_sym;
                curr_sym = div;
                if vec![add, mul, div, sub].contains(&prev_sym) {
                    return Err(SyntaxError {
                        index: index,
                        message: String::from("Two operators must have an operand between them"),
                    });
                }
                if prev_sym == dec {
                    return Err(SyntaxError {
                        index: index,
                        message: String::from("A number can't end with a decimal point."),
                    });
                }
                if prev_sym == opbr {
                    return Err(SyntaxError {
                        index: index,
                        message: String::from("First operand is missing."),
                    });
                }
            }
            '(' => {
                prev_sym = curr_sym;
                curr_sym = opbr;
                if prev_sym == clbr || prev_sym == dig {
                    return Err(SyntaxError {
                        index: index,
                        message: String::from("Two operands must have an operator between them."),
                    });
                }
                if prev_sym == dec {
                    return Err(SyntaxError {
                        index: index,
                        message: String::from("A number can't end with a decimal point."),
                    });
                }
            }
            ')' => {
                prev_sym = curr_sym;
                curr_sym = clbr;
                if prev_sym == dec {
                    return Err(SyntaxError {
                        index: index,
                        message: String::from("A number can't end with a decimal point."),
                    });
                }
                if prev_sym == opbr {
                    return Err(SyntaxError {
                        index: index,
                        message: String::from("Empty braces Found."),
                    });
                }
                if vec![add, mul, div, sub].contains(&prev_sym) {
                    return Err(SyntaxError {
                        index: index,
                        message: String::from("2nd operand is missing."),
                    });
                }
            }
            '0'..='9' => {
                prev_sym = curr_sym;
                curr_sym = dig;
                if prev_sym == clbr {
                    return Err(SyntaxError {
                        index: index,
                        message: String::from("Operator is missing"),
                    });
                }
            }
            '.' => {
                prev_sym = curr_sym;
                curr_sym = dec;
                if prev_sym == clbr {
                    return Err(SyntaxError {
                        index: index,
                        message: String::from("Decimal point found at inappropriate position"),
                    });
                }
            }
            _ => {}
        }

        if index == result_length - 1 {
            if curr_sym == dec {
                return Err(SyntaxError {
                    index: index,
                    message: String::from("Decimal at inappropriate position."),
                });
            }
            if vec![add, mul, div, sub].contains(&curr_sym) {
                return Err(SyntaxError {
                    index: index,
                    message: String::from("2nd operator is missing."),
                });
            }
        }
    }
    //replacing (- with (0-  and  (+ with (0+
    let mut new_result = String::new();
    let (mut start, mut end) = (0, 0);
    for i in no_rh_oprnd_minus_pos {
        end = i;
        new_result.push_str(&result[start..end]);
        new_result.push_str("0");
        start = end;
    }
    new_result.push_str(&result[start..result.len()]);
    //result fully replaces (- with (0-  and  (+ with (0+
    Ok(new_result)
}
/*
//Writing an expression parser in rust
//Goal is to parse the string expression to an Expression Tree(Node in this case)
//How to do this:
//a single function `create_node_from_str` will be implemented
//the fn takes the string argument and generates the complete expression tree
//the fn dissamble the string into smallest uppermost possible node and
//recursively calls itself to pass the redundant string into the immediate
//child node.
//Procedure:
//1. denude the expression by removing top level unnecessary brackets
//      if an expression is denudable is determined by
//      1. presence of paranthesis(p1) at start
//      2. presence of complement paranthesis of p1 at the end
//   then we should remove the paranthesis at the start and at the end
/*  procedure:
    if starts and ends with braces
    get a slice of 1..length of s to iterate throught
    at any moment number of closing braces cant be higher than opening braces
    check if number of opening and closing braces are equal

*/
//
//2. dissociate the string into an vector using most-top-level operator
//      2*(4+7)/9 => [2,*,(4+7),/,9]
//3. if there is more than one top level operation we have convert it into one
//   by correctly wrapping the right expression(of greater precedence) with brackets
//   this step could be recursively used until there is only one expression left
//      2*(4+7)-20/7:Vec
//    =>2*(4+7)-(20/7):Vec
//    =>(2*(4+7))-(20/7):Vec
//4. Now we have a vector that contains only one operator and two operand string
//   either of them could be a number.
//   Now if the operand is a number we will parse it to float and make an Number(f)
//   if the operand is an expression we will get the node from `create_node_from_str`
//   and then make Expression(node)
//   after it parses the operator to Operaton enum
//   finally it combines all 3 to create a node
//   then it returns the node
*/
fn is_denudable(exp: &String) -> bool {
    //checks if the exp is denudable
    let mut byte_arr = exp.as_bytes();
    let length = exp.len();
    if byte_arr[0] != b'(' || byte_arr[length - 1] != b')' {
        return false;
    }
    let (mut opb, mut clb) = (0, 0);
    for (i, c) in exp.chars().enumerate() {
        if i != 0 && i != length - 1 {
            match c {
                '(' => {
                    opb += 1;
                }
                ')' => {
                    clb += 1;
                    if clb > opb {
                        //Wrong peak
                        return false;
                    }
                }
                _ => {}
            }
        }
    }
    if opb != clb {
        return false;
    };
    true
}
fn denude(expression: &String) -> String {
    let mut denuded_exp = String::new();
    if is_denudable(expression) {
        denuded_exp = expression[1..(expression.len() - 1)].to_string();
        while is_denudable(&denuded_exp) {
            denuded_exp = denuded_exp[1..(denuded_exp.len() - 1)].to_string();
        }
        denuded_exp
    } else {
        expression.to_string()
    }
}

fn top_level_disassembler(s: &String) -> Vec<String> {
    let bytes = s.as_bytes();
    let length = bytes.len();
    let mut level = 0;
    let mut i = 0;
    let mut loc = vec![];
    let mut result = vec![];
    while i < length {
        let c = bytes[i] as char;
        match c {
            '(' => {
                level -= 1;
            }
            ')' => {
                level += 1;
            }
            '+' | '-' | '*' | '/' => {
                if level == 0 {
                    loc.push(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    let (mut splitstart, mut splitend) = (0, 0);
    for i in loc {
        splitend = i;
        result.push(s[splitstart..splitend].to_string());
        result.push((bytes[splitend] as char).to_string());
        splitstart = i + 1;
    }
    result.push(s[splitstart..length].to_string());

    return result;
}
fn top_level_assembler(v: &Vec<String>) -> Vec<String> {
    let order = vec!["/", "*", "-", "+"];
    let mut wop = 0;
    let l = v.len();
    let mut broken = false;
    for i in order {
        let mut index = 0;
        while index < l {
            if v[index].eq(&String::from(i)) {
                wop = index;
                broken = true;
                break;
            }
            index += 1;
        }
        if broken {
            break;
        }
    }
    let mut merge_start = wop - 1;
    let mut i = 0;
    let mut result = vec![];
    while i < l {
        if i == merge_start {
            let mut s = String::from("(");
            s.push_str(v[i].as_str());
            s.push_str(v[i + 1].as_str());
            s.push_str(v[i + 2].as_str());
            s.push_str(")");
            result.push(s);
            i += 3;
            continue;
        } else {
            result.push(v[i].to_string());
            i += 1;
        }
    }
    result
}
fn top_level_assembler_iter(v: &Vec<String>) -> Vec<String> {
    let mut result = vec![];
    if v.len() > 3 {
        result = top_level_assembler(v);
    } else {
        return v.clone();
    }
    while result.len() > 3 {
        result = top_level_assembler(&result);
    }
    result
}
fn is_digit(s: &String) -> bool {
    match s.as_bytes()[0] as char {
        '0'..='9' => true,
        _ => false,
    }
}

fn create_node_from_str(s: &String) -> Node {
    let denuded = denude(&s);
    let disassmabled = top_level_disassembler(&denuded);
    let assembled = top_level_assembler_iter(&disassmabled);
    let (mut oprnd1, mut oprnd2) = (
        Operand::Number(Box::new(0.0)),
        Operand::Number(Box::new(0.0)),
    );
    if is_digit(&assembled[0]) {
        let f: f64 = assembled[0].parse().unwrap();
        oprnd1 = Operand::Number(Box::new(f));
    } else {
        oprnd1 = Operand::Expression(Box::new(create_node_from_str(&assembled[0])));
    }
    if assembled.len() == 1 {
        return Node {
            operator: Operator::Add,
            operand1: oprnd1,
            operand2: Operand::Number(Box::new(0.0)),
        };
    }
    if is_digit(&assembled[2]) {
        let f: f64 = assembled[2].parse().unwrap();
        oprnd2 = Operand::Number(Box::new(f));
    } else {
        oprnd2 = Operand::Expression(Box::new(create_node_from_str(&assembled[2])));
    }
    let mut operator = Operator::Add;
    match assembled[1].as_str() {
        "+" => operator = Operator::Add,
        "-" => operator = Operator::Sub,
        "*" => operator = Operator::Mul,
        "/" => operator = Operator::Div,
        _ => {}
    }
    Node {
        operator: operator,
        operand1: oprnd1,
        operand2: oprnd2,
    }
}
fn evaluate(exp: &String) -> Result<String, SyntaxError> {
    let x = preprocess(exp);
    match x {
        Ok(ppd) => {
            Ok(create_node_from_str(&ppd).calculate().to_string())
        }
        Err(e) => {
            Err(e)
        }
    }
}
use std::io::stdin;
fn main() {
    /*
    loop {
        let mut x = String::new();
        stdin().read_line(&mut x);
        let p = evaluate(&x);
        match p {
            Ok(r) => {
                println!("Result: {}", r)
            }
            Err(e) => {
                println!("Error at {}: {}", e.index, e.message)
            }
        }
    }
    */
    let f:f64 = "1.".parse().unwrap();
    println!("{}", f);
}
