#[derive(Debug)]
pub struct AST {
    pub main: Function,
}

#[derive(Debug)]
pub struct Function {
    pub function_decl: FunctionDecl,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct FunctionDecl {
    pub name: Symbol,
    pub inputs: Vec<Parameter>,
    pub outputs: Vec<DataType>, 
}

#[derive(Debug)]
pub struct Parameter {
    pub data_type: DataType,
    pub value: Expression,
}

#[derive(Debug)]
pub enum Statement {
    Function(Function),
    DataTypeDef(DataType),
    Expression(Expression),
    Assignment(Assignment),
    Conditional(Conditional),
    Loop(Loop),
    Print(Symbol),
    Return(Return),
}

#[derive(Debug)]
pub enum Expression {
    Constant(Constant),
    FunctionCall,
    BinaryExpression(BinaryExpression),
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
    pub operator: BinaryOperator,
}

#[derive(Debug)]
pub enum DataType {
    Enum,
    Struct,
}

#[derive(Debug)]
pub enum Loop {
    ForEach,
    While,
}

#[derive(Debug)]
pub struct Assignment {
    pub lhs: Vec<Symbol>,
    pub rhs: Expression,
}

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
}

#[derive(Debug)]
pub struct Conditional {
    pub if_branches: Vec<ConditionalBranch>,
    pub else_branch: Vec<Statement>,
}

#[derive(Debug)]
pub struct ConditionalBranch {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct Return {
    pub value: Expression,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Addition,
    Subtraction,
    Division,
    Multiplication,
    Mod,
}

#[derive(Debug)]
pub enum Constant {
    Integer(i64),
}





