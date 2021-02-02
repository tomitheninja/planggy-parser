use super::Expression;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Operation {
    Parentheses(Box<Expression>), // (exp)
    Abs(Box<Expression>),         // |exp|
    // ItemAt(Box<Expression>),                  // arr[idx]
    // Slice(Box<Expression>, Box<Expression>),  // arr[start..end]
    Search(Box<Expression>, Box<Expression>), // arr @ item
    Not(Box<Expression>),                     // NEM <exp:boolean>
    Neg(Box<Expression>),                     // - <exp:boolean>
    Rnd(Box<Expression>),                     // RND <exp:int ~ upper limit>
    Sin(Box<Expression>),                     // SIN <exp:float ~ radian>
    Cos(Box<Expression>),                     // COS <exp:float ~ radian>
    Tan(Box<Expression>),                     // TAN <exp:float ~ radian>
    Arcsin(Box<Expression>),                  // SIN <exp:float ~ radian>
    Arccos(Box<Expression>),                  // SIN <exp:float ~ radian>
    Log(Box<Expression>),                     // SIN <exp:float>
    Exp(Box<Expression>),                     // SIN <exp:float>
    ToFloat(Box<Expression>),                 // VALOS <exp:int>
    Floor(Box<Expression>),                   // EGESZ <exp:float>
    Round(Box<Expression>),                   // KEREK <exp:float>
    ToUpper(Box<Expression>),                 // NAGY <exp:char>
    ToLower(Box<Expression>),                 // KIS <exp:char>
    IsWordChar(Box<Expression>),              // BETU <exp:char> -> matches [a-zA-Z]?
    IsDigit(Box<Expression>),                 // SZAM <exp:char> -> matches [0-9]?
    Pow(Box<Expression>, Box<Expression>),    // <exp:float> ^ <exp:float>
    Mul(Box<Expression>, Box<Expression>),    // <exp:int|float> * <exp:int|float>
    Div(Box<Expression>, Box<Expression>),    // <exp:float> / <exp:float>
    DivInt(Box<Expression>, Box<Expression>), // <exp:int> DIV <exp:int>
    Mod(Box<Expression>, Box<Expression>),    // <exp:int> MOD <exp:int>
    Add(Box<Expression>, Box<Expression>),    // <exp:int|float|string> + <exp:int|string&char>
    Sub(Box<Expression>, Box<Expression>),    // <exp:int|float> + <exp:int|float>
    And(Box<Expression>, Box<Expression>),    // <exp:logical> ES <exp:logical>
    Or(Box<Expression>, Box<Expression>),     // <exp:logical> VAGY <exp:logical>
    Eq(Box<Expression>, Box<Expression>),     // <exp:any> = <exp:any>
    Ne(Box<Expression>, Box<Expression>),     // <exp:any> /= <exp:any>
    Lt(Box<Expression>, Box<Expression>),     // <exp:any> < <exp:any>
    Le(Box<Expression>, Box<Expression>),     // <exp:any> <= <exp:any>
    Gt(Box<Expression>, Box<Expression>),     // <exp:any> > <exp:any>
    Ge(Box<Expression>, Box<Expression>),     // <exp:any> >= <exp:any>
}
