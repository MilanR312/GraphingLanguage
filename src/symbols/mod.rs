use std::collections::{BTreeMap, HashMap};

use crate::{ids::{FunctionId, VariableId}, parser::{block::BlockExpression, expression::Expression, pattern::Pattern, statement::Statement, Program}, spanned::Spanned};

#[derive(PartialEq, Eq, Debug, Hash, salsa::Update, Clone, Copy)]
pub enum ScopeId<'db>{
    Program(Program<'db>),
    Block(BlockExpression<'db>)
}


/// tracks reference to each scopes parent
#[salsa::tracked(debug)]
pub struct ScopeParentTable<'db>{
    #[returns(ref)]
    pub table: Vec<(ScopeId<'db>, ScopeId<'db>)>
}

#[salsa::tracked]
pub fn create_scope_parent_table<'db>(db: &'db dyn salsa::Database, program: Program<'db>) -> ScopeParentTable<'db> {
    println!("create parent table for program");
    let mut map = Vec::new();
    for statement in program.statements(db) {
        let body = match statement {
            Statement::Function(x) => &**x.body(db),
            Statement::Variable(x) => &*x.body(db)
        };
        build_scope_parent_table_expression(db, body, Some(ScopeId::Program(program)), &mut map);

    }
    ScopeParentTable::new(db, map)
}
fn build_scope_parent_table_expression<'db>(
    db: &'db dyn salsa::Database,
    expression: &Expression<'db>,
    parent: Option<ScopeId<'db>>,
    map: &mut Vec<(ScopeId<'db>, ScopeId<'db>)>
){
    match expression {
        Expression::Block(x) => build_scope_parent_table(db, *x, parent, map),
        Expression::Binary(l, _, r) => {
            build_scope_parent_table_expression(db, &**l, parent, map);
            build_scope_parent_table_expression(db, &**r, parent, map);
        },
        _ => {}
    }
}
fn build_scope_parent_table<'db>(
    db: &'db dyn salsa::Database,
    node: BlockExpression<'db>,
    parent: Option<ScopeId<'db>>,
    map: &mut Vec<(ScopeId<'db>, ScopeId<'db>)>
) {
    if let Some(parent) = parent {
        map.push((ScopeId::Block(node), parent));
    }
    for statement in node.statements(db) {
        let body = match &**statement {
            Statement::Function(x) => &**x.body(db),
            Statement::Variable(x) => &*x.body(db)
        };
        build_scope_parent_table_expression(db,body, parent, map);
    }
}

#[salsa::tracked]
pub fn get_parent_scope<'db>(db: &'db dyn salsa::Database, scope: ScopeId<'db>, table: ScopeParentTable<'db>) -> Option<ScopeId<'db>> {
    let table = table.table(db);
    table.iter()
        .find(|x| x.0 == scope)
        .map(|x| x.1)
}

#[salsa::tracked(debug)]
pub struct SymbolNode<'db>{
    #[returns(ref)]
    pub functions: Vec<FunctionId<'db>>,
    #[returns(ref)]
    pub variables: Vec<VariableId<'db>>,
}
#[salsa::tracked(debug)]
pub struct SymbolTable<'db>{
    #[returns(ref)]
    pub items: Vec<(ScopeId<'db>, SymbolNode<'db>)>
}


#[salsa::tracked]
pub fn create_symbol_table<'db>(db: &'db dyn salsa::Database, program: Program<'db>) -> SymbolTable<'db> {
    println!("creating symbol table for program");
    let mut items = Vec::new();
    let mut functions = vec![];
    let mut variables = vec![];

    for statement in program.statements(db) {
        let body = match statement {
            Statement::Function(x) => {
                functions.push(*x.name(db));
                &**x.body(db)
            },
            Statement::Variable(x) => {
                let pattern = &**x.name(db);
                build_symbol_table_pattern(db, pattern, &mut variables);
                &*x.body(db)
            }
        };
        build_symbol_table_expression(db, body, &mut items);

    }
    items.push((ScopeId::Program(program), SymbolNode::new(db, functions, variables)));

    SymbolTable::new(db, items)
}
fn build_symbol_table<'db>(
    db: &'db dyn salsa::Database,
    id: ScopeId<'db>,
    statements: &[Spanned<Statement<'db>>],
    map: &mut Vec<(ScopeId<'db>, SymbolNode<'db>)>
) {
    let mut functions = vec![];
    let mut variables = vec![];
    for statement in statements {
        let body = match &**statement {
            Statement::Function(x) => {
                functions.push(*x.name(db));
                &**x.body(db)
            },
            Statement::Variable(x) => {
                let pattern = &**x.name(db);
                build_symbol_table_pattern(db, pattern, &mut variables);
                &*x.body(db)
            }
        };
        build_symbol_table_expression(db, body, map);
    }
    map.push((id, SymbolNode::new(db, functions, variables)));
}
fn build_symbol_table_pattern<'db>(
    db: &'db dyn salsa::Database,
    pattern: &Pattern<'db>,
    variables: &mut Vec<VariableId<'db>>
){
    match pattern {
        Pattern::Variable(x) => variables.push(**x),
        Pattern::Tuple(x) => x.iter().for_each(|ell| build_symbol_table_pattern(db, &**ell, variables)),
        _ => {}
    }
}
fn build_symbol_table_expression<'db>(
    db: &'db dyn salsa::Database,
    expression: &Expression<'db>,
    map: &mut Vec<(ScopeId<'db>, SymbolNode<'db>)>
){
    match expression {
        Expression::Block(x) => build_symbol_table(db, ScopeId::Block(*x), x.statements(db), map),
        Expression::Binary(l, _, r) => {
            build_symbol_table_expression(db, &**l, map);
            build_symbol_table_expression(db, &**r, map);
        },
        _ => {}
    }
}


/*




#[salsa::tracked(debug)]
pub struct SymbolNode<'db>{
    functions: Vec<FunctionId<'db>>,
    variables: Vec<VariableId<'db>>,
}

#[salsa::tracked]
pub fn create_symbol_table_for_scope<'db>(db: &'db dyn salsa::Database, scope: BlockExpression<'db>) -> SymbolTable<'db> {
    todo!()
}
fn create_symbol_table_for_expression<'db>(db: &'db dyn salsa::Database, expr: &Expression<'db>) -> Option<SymbolTable<'db>> {
    match expr {
        Expression::Block(x) => Some(create_symbol_table_for_scope(db, *x)),
        _ => None
    }
}
#[salsa::tracked]
pub fn create_symbol_table_for_file_source<'db>(db: &'db dyn salsa::Database, scope: Program<'db>) -> SymbolTable<'db> {
    let content = scope.statements(db);
    let mut functions = vec![];
    let mut variables = vec![];
    for statement in content {
        match statement {
            Statement::Function(x) => {
                functions.push(*x.name(db));
                let body = &**x.body(db);
                let st = create_symbol_table_for_expression(db, body);
            },
            Statement::Variable(x) => {

            }
        }

    }
    SymbolTable::new(db, functions, variables)
}
*/