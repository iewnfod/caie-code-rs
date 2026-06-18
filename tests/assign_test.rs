use caie_code_rs::{Expr, Interpreter, Op, RuntimeValue, Stmt, Type};

#[test]
fn assign() {
	// 手动创建 AST
	let mock_ast = Stmt::Block {
		stmts: vec![
			Stmt::VarDecl {
				name: "X".to_string(),
				var_type: Type::Int,
				span: None,
			},
			Stmt::Assign {
				name: "X".to_string(),
				value: Expr::Binary {
					left: Box::new(Expr::Literal {value: RuntimeValue::Int(10), span: None}),
					op: Op::Add,
					right: Box::new(Expr::Literal {value: RuntimeValue::Int(5), span: None}),
					span: None,
				},
				span: None,
			},
			Stmt::Print {
				value: vec![
					Expr::Literal {
						value: RuntimeValue::Str("X".to_string()), span: None,
					},
					Expr::Literal {
						value: RuntimeValue::Str("=".to_string()), span: None,
					},
					Expr::Get {
						name: "X".to_string(),
						span: None,
					}
				],
				span: None,
			},
		],
		span: None,
	};

	// 执行内核逻辑
	let mut interpreter = Interpreter::debug();
	interpreter.execute(mock_ast);
}
