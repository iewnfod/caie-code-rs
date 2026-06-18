use caie_code_rs::{Expr, Interpreter, Op, RuntimeValue, Stmt, Type};

#[test]
fn for_loop() {
	// 手动创建 AST
	let mock_ast = Stmt::Block {
		stmts: vec![
			Stmt::For {
				var_name: "i".to_string(),
				start: Expr::Literal { value: RuntimeValue::Int(1), span: None },
				end: Expr::Literal { value: RuntimeValue::Int(10), span: None },
				body: Box::new(Stmt::Print {
					value: vec![Expr::Get {
						name: "i".to_string(),
						span: None,
					}],
					span: None,
				}),
				span: None,
			},
		],
		span: None,
	};

	// 执行内核逻辑
	let mut interpreter = Interpreter::debug();
	interpreter.execute(mock_ast);
}

#[test]
fn while_loop() {
	let mock_ast = Stmt::Block {
		stmts: vec![
			Stmt::VarDecl {
				name: "X".to_string(),
				var_type: Type::Int,
				span: None,
			},
			Stmt::While {
				body: Box::new(Stmt::Block {
					stmts: vec![
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
						Stmt::Assign {
							name: "X".to_string(),
							value: Expr::Binary {
								left: Box::new(Expr::Get {
									name: "X".to_string(),
									span: None,
								}),
								op: Op::Add,
								right: Box::new(Expr::Literal { value: RuntimeValue::Int(1), span: None }),
								span: None,
							},
							span: None,
						},
					],
					span: None,
				}),
				condition: Expr::Binary {
					left: Box::new(Expr::Get {
						name: "X".to_string(),
						span: None,
					}),
					op: Op::Lt,
					right: Box::new(Expr::Literal { value: RuntimeValue::Int(5), span: None }),
					span: None,
				},
				span: None,
			},
		],
		span: None,
	};

	let mut interpreter = Interpreter::debug();
	interpreter.execute(mock_ast);
}

#[test]
fn repeat_loop() {
	let mock_ast = Stmt::Block {
		stmts: vec![
			Stmt::VarDecl {
				name: "X".to_string(),
				var_type: Type::Int,
				span: None,
			},
			Stmt::Repeat {
				body: Box::new(Stmt::Block {
					stmts: vec![
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
						Stmt::Assign {
							name: "X".to_string(),
							value: Expr::Binary {
								left: Box::new(Expr::Get {
									name: "X".to_string(),
									span: None,
								}),
								op: Op::Add,
								right: Box::new(Expr::Literal { value: RuntimeValue::Int(1), span: None }),
								span: None,
							},
							span: None,
						},
					],
					span: None,
				}),
				condition: Expr::Binary {
					left: Box::new(Expr::Get {
						name: "X".to_string(),
						span: None,
					}),
					op: Op::Gt,
					right: Box::new(Expr::Literal { value: RuntimeValue::Int(5), span: None }),
					span: None,
				},
				span: None,
			},
		],
		span: None,
	};

	let mut interpreter = Interpreter::debug();
	interpreter.execute(mock_ast);
}
