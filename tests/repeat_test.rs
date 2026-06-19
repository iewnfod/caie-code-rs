use caie_code_rs::{Expr, Interpreter, Op, RuntimeValue, Stmt, Type};

/// 测试 1：REPEAT...UNTIL 基本（无 continue）
///   I <- 0
///   REPEAT
///       OUTPUT I
///       I <- I + 1
///   UNTIL I >= 3
/// 预期输出: 0 1 2  （输出 I 后自增，I==3 时退出）
#[test]
fn repeat_basic() {
	let ast = Stmt::Block {
		stmts: vec![
			Stmt::VarDecl { name: "I".to_string(), var_type: Type::Int, span: None },
			Stmt::Assign {
				name: "I".to_string(),
				value: Expr::Literal { value: RuntimeValue::Int(0), span: None },
				span: None,
			},
			Stmt::Repeat {
				body: Box::new(Stmt::Block {
					stmts: vec![
						Stmt::Print {
							value: vec![Expr::Get { name: "I".to_string(), span: None }],
							span: None,
						},
						Stmt::Assign {
							name: "I".to_string(),
							value: Expr::Binary {
								left: Box::new(Expr::Get { name: "I".to_string(), span: None }),
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
					left: Box::new(Expr::Get { name: "I".to_string(), span: None }),
					op: Op::Gte,
					right: Box::new(Expr::Literal { value: RuntimeValue::Int(3), span: None }),
					span: None,
				},
				span: None,
			},
		],
		span: None,
	};

	print!("\n[repeat_basic] 预期: 0 1 2\n[repeat_basic] 实际: ");
	let mut interp = Interpreter::new();
	interp.execute(ast);
	println!();
}

/// 测试 2：REPEAT 里的 CONTINUE 不死循环（关键回归测试）
///   I <- 0
///   REPEAT
///       I <- I + 1
///       IF I == 3 THEN CONTINUE ENDIF
///       OUTPUT I
///   UNTIL I >= 5
/// 预期输出: 1 2 4 5
///   - I=1,2: 正常输出
///   - I=3: continue 跳过 OUTPUT，但 UNTIL I>=5 仍检查（不退出）
///   - I=4: 输出 4
///   - I=5: 输出 5，UNTIL I>=5 成立，退出
/// 注意：3 不会被输出。如果 repeat_stmt 的 continue 错误地跳过 condition，
/// 这个测试会死循环（I 卡在 3，永远 < 5）。
#[test]
fn repeat_continue_no_infinite_loop() {
	let ast = Stmt::Block {
		stmts: vec![
			Stmt::VarDecl { name: "I".to_string(), var_type: Type::Int, span: None },
			Stmt::Assign {
				name: "I".to_string(),
				value: Expr::Literal { value: RuntimeValue::Int(0), span: None },
				span: None,
			},
			Stmt::Repeat {
				body: Box::new(Stmt::Block {
					stmts: vec![
						Stmt::Assign {
							name: "I".to_string(),
							value: Expr::Binary {
								left: Box::new(Expr::Get { name: "I".to_string(), span: None }),
								op: Op::Add,
								right: Box::new(Expr::Literal { value: RuntimeValue::Int(1), span: None }),
								span: None,
							},
							span: None,
						},
						Stmt::If {
							condition: Expr::Binary {
								left: Box::new(Expr::Get { name: "I".to_string(), span: None }),
								op: Op::Eq,
								right: Box::new(Expr::Literal { value: RuntimeValue::Int(3), span: None }),
								span: None,
							},
							true_body: Box::new(Stmt::Continue { span: None }),
							false_body: None,
							span: None,
						},
						Stmt::Print {
							value: vec![Expr::Get { name: "I".to_string(), span: None }],
							span: None,
						},
					],
					span: None,
				}),
				condition: Expr::Binary {
					left: Box::new(Expr::Get { name: "I".to_string(), span: None }),
					op: Op::Gte,
					right: Box::new(Expr::Literal { value: RuntimeValue::Int(5), span: None }),
					span: None,
				},
				span: None,
			},
		],
		span: None,
	};

	print!("\n[repeat_continue_no_infinite_loop] 预期: 1 2 4 5\n[repeat_continue_no_infinite_loop] 实际: ");
	let mut interp = Interpreter::new();
	interp.execute(ast);
	println!();
}

/// 测试 3：REPEAT + CONTINUE + UNTIL TRUE（最容易死循环的组合）
///   I <- 0
///   REPEAT
///       I <- I + 1
///       IF I == 2 THEN CONTINUE ENDIF
///       OUTPUT I
///   UNTIL I >= 4
/// 预期输出: 1 3 4
///   - I=1: 输出 1
///   - I=2: continue 跳过输出；UNTIL I>=4 不成立，继续
///   - I=3: 输出 3
///   - I=4: 输出 4；UNTIL I>=4 成立，退出
#[test]
fn repeat_continue_until_condition() {
	let ast = Stmt::Block {
		stmts: vec![
			Stmt::VarDecl { name: "I".to_string(), var_type: Type::Int, span: None },
			Stmt::Assign {
				name: "I".to_string(),
				value: Expr::Literal { value: RuntimeValue::Int(0), span: None },
				span: None,
			},
			Stmt::Repeat {
				body: Box::new(Stmt::Block {
					stmts: vec![
						Stmt::Assign {
							name: "I".to_string(),
							value: Expr::Binary {
								left: Box::new(Expr::Get { name: "I".to_string(), span: None }),
								op: Op::Add,
								right: Box::new(Expr::Literal { value: RuntimeValue::Int(1), span: None }),
								span: None,
							},
							span: None,
						},
						Stmt::If {
							condition: Expr::Binary {
								left: Box::new(Expr::Get { name: "I".to_string(), span: None }),
								op: Op::Eq,
								right: Box::new(Expr::Literal { value: RuntimeValue::Int(2), span: None }),
								span: None,
							},
							true_body: Box::new(Stmt::Continue { span: None }),
							false_body: None,
							span: None,
						},
						Stmt::Print {
							value: vec![Expr::Get { name: "I".to_string(), span: None }],
							span: None,
						},
					],
					span: None,
				}),
				condition: Expr::Binary {
					left: Box::new(Expr::Get { name: "I".to_string(), span: None }),
					op: Op::Gte,
					right: Box::new(Expr::Literal { value: RuntimeValue::Int(4), span: None }),
					span: None,
				},
				span: None,
			},
		],
		span: None,
	};

	print!("\n[repeat_continue_until_condition] 预期: 1 3 4\n[repeat_continue_until_condition] 实际: ");
	let mut interp = Interpreter::new();
	interp.execute(ast);
	println!();
}
