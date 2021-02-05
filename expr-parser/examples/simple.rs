use expr_parser::Expr;

fn main() {
    let result = Expr::parse("1 + 2");

    let result = result.unwrap().unwrap_binary();
    let op = result.op_code();
    let lhs = result.lhs().clone().unwrap_value().unwrap_int();
    let rhs = result.rhs().clone().unwrap_value().unwrap_int();
    print!("{} {} {}", lhs, op, rhs);
}
