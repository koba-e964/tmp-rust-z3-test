extern crate z3;

use z3::{Ast, Config, Context, Solver, Sort, Symbol};

fn main() {
    let context = Context::new(&Config::new());

    // x + 1 = 2
    let x = Symbol::from_string(&context, "x");
    let x = Ast::new_const(&x, &Sort::int(&context)); 
    let one = Ast::from_i64(&context, 1);
    let two = Ast::from_i64(&context, 2);
    let ast = x.add(&[&one])._eq(&two);
    println!("ast = {}", ast);

    // x + 1 = 3
    let three = Ast::from_i64(&context, 3);
    let ast2 = x.add(&[&one])._eq(&three);
    println!("ast2 = {}", ast2);
    
    // Solver
    let solver = Solver::new(&context);
    solver.assert(&ast);
    println!("SAT = {}", solver.check());
    let model = solver.get_model();
    println!("x = {}", model.eval(&x).unwrap());
}
