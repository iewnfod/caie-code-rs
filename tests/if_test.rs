use caie_code_rs::{Expr, Interpreter, Op, RuntimeValue, Stmt};

#[test]
fn if_stmt() {
	// 手动创建 AST
	let mock_ast = Stmt::Block {
		stmts: vec![
			Stmt::If {
				condition: Expr::Binary {
					left: Box::new(Expr::Literal { value: RuntimeValue::Bool(true), span: None }),
					op: Op::And,
					right: Box::new(Expr::Literal { value: RuntimeValue::Bool(false), span: None }),
					span: None
				},
				true_body: Box::new(Stmt::Print { value: vec![
					Expr::Literal { value: RuntimeValue::Str("TRUE AND FALSE = TRUE".to_string()), span: None }
				], span: None }),
				false_body: Some(Box::new(Stmt::Print { value: vec![
					Expr::Literal { value: RuntimeValue::Str("TRUE AND FALSE = FALSE".to_string()), span: None }
				], span: None })),
				span: None,
			},
			Stmt::If {
				condition: Expr::Binary {
					left: Box::new(Expr::Binary {
						left: Box::new(Expr::Literal { value: RuntimeValue::Int(1), span: None }),
						op: Op::Add,
						right: Box::new(Expr::Literal { value: RuntimeValue::Int(2), span: None }),
						span: None
					}),
					op: Op::Eq,
					right: Box::new(Expr::Literal { value: RuntimeValue::Int(3), span: None }),
					span: None
				},
				true_body: Box::new(Stmt::Print { value: vec![
					Expr::Literal { value: RuntimeValue::Str("1 + 2 = 3".to_string()), span: None }
				], span: None }),
				false_body: Some(Box::new(Stmt::Print { value: vec![
					Expr::Literal { value: RuntimeValue::Str("1 + 2 != 3".to_string()), span: None }
				], span: None })),
				span: None,
			},
		],
		span: None,
	};

	// 执行内核逻辑
	let mut interpreter = Interpreter::debug();
	interpreter.execute(mock_ast);
}
