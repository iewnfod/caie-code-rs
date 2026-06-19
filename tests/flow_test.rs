use caie_code_rs::{Expr, Interpreter, Op, RuntimeValue, Stmt, Type};

/// 测试 1：FOR 循环里的 BREAK
///   FOR i <- 1 TO 10
///       IF i == 3 THEN BREAK ENDIF
///       OUTPUT i
///   NEXT i
/// 预期输出: 1 2  （i=3 时 break 跳出）
#[test]
fn for_break() {
	let ast = Stmt::Block {
		stmts: vec![
			Stmt::For {
				var_name: "i".to_string(),
				start: Expr::Literal { value: RuntimeValue::Int(1), span: None },
				end: Expr::Literal { value: RuntimeValue::Int(10), span: None },
				body: Box::new(Stmt::Block {
					stmts: vec![
						Stmt::If {
							condition: Expr::Binary {
								left: Box::new(Expr::Get { name: "i".to_string(), span: None }),
								op: Op::Eq,
								right: Box::new(Expr::Literal { value: RuntimeValue::Int(3), span: None }),
								span: None,
							},
							true_body: Box::new(Stmt::Break { span: None }),
							false_body: None,
							span: None,
						},
						Stmt::Print {
							value: vec![Expr::Get { name: "i".to_string(), span: None }],
							span: None,
						},
					],
					span: None,
				}),
				span: None,
			},
		],
		span: None,
	};

	print!("\n[for_break] 预期: 1 2\n[for_break] 实际: ");
	let mut interp = Interpreter::new();
	interp.execute(ast);
	println!();
}

/// 测试 2：FOR 循环里的 CONTINUE
///   FOR i <- 1 TO 5
///       IF i == 3 THEN CONTINUE ENDIF
///       OUTPUT i
///   NEXT i
/// 预期输出: 1 2 4 5  （3 被 continue 跳过输出）
#[test]
fn for_continue() {
	let ast = Stmt::Block {
		stmts: vec![
			Stmt::For {
				var_name: "i".to_string(),
				start: Expr::Literal { value: RuntimeValue::Int(1), span: None },
				end: Expr::Literal { value: RuntimeValue::Int(5), span: None },
				body: Box::new(Stmt::Block {
					stmts: vec![
						Stmt::If {
							condition: Expr::Binary {
								left: Box::new(Expr::Get { name: "i".to_string(), span: None }),
								op: Op::Eq,
								right: Box::new(Expr::Literal { value: RuntimeValue::Int(3), span: None }),
								span: None,
							},
							true_body: Box::new(Stmt::Continue { span: None }),
							false_body: None,
							span: None,
						},
						Stmt::Print {
							value: vec![Expr::Get { name: "i".to_string(), span: None }],
							span: None,
						},
					],
					span: None,
				}),
				span: None,
			},
		],
		span: None,
	};

	print!("\n[for_continue] 预期: 1 2 4 5\n[for_continue] 实际: ");
	let mut interp = Interpreter::new();
	interp.execute(ast);
	println!();
}

/// 测试 3：WHILE 循环里的 BREAK
///   Count <- 0
///   WHILE TRUE
///       Count <- Count + 1
///       IF Count == 4 THEN BREAK ENDIF
///   ENDWHILE
///   OUTPUT Count       // 预期 4
#[test]
fn while_break() {
	let ast = Stmt::Block {
		stmts: vec![
			Stmt::VarDecl { name: "Count".to_string(), var_type: Type::Int, span: None },
			Stmt::Assign {
				name: "Count".to_string(),
				value: Expr::Literal { value: RuntimeValue::Int(0), span: None },
				span: None,
			},
			Stmt::While {
				condition: Expr::Literal { value: RuntimeValue::Bool(true), span: None },
				body: Box::new(Stmt::Block {
					stmts: vec![
						Stmt::Assign {
							name: "Count".to_string(),
							value: Expr::Binary {
								left: Box::new(Expr::Get { name: "Count".to_string(), span: None }),
								op: Op::Add,
								right: Box::new(Expr::Literal { value: RuntimeValue::Int(1), span: None }),
								span: None,
							},
							span: None,
						},
						Stmt::If {
							condition: Expr::Binary {
								left: Box::new(Expr::Get { name: "Count".to_string(), span: None }),
								op: Op::Eq,
								right: Box::new(Expr::Literal { value: RuntimeValue::Int(4), span: None }),
								span: None,
							},
							true_body: Box::new(Stmt::Break { span: None }),
							false_body: None,
							span: None,
						},
					],
					span: None,
				}),
				span: None,
			},
			Stmt::Print {
				value: vec![Expr::Get { name: "Count".to_string(), span: None }],
				span: None,
			},
		],
		span: None,
	};

	print!("\n[while_break] 预期: 4\n[while_break] 实际: ");
	let mut interp = Interpreter::new();
	interp.execute(ast);
	println!();
}

/// 测试 4：函数内部的 BREAK 不能穿透到调用点的循环
///   （验证 call_func 拦截 Break，不外传）
///   PROCEDURE Inner()       // 这里仍当作 Function 建模（return_type=Int）
///       BREAK               // 函数体里有裸 break（语义错误，但不应影响外层）
///   ENDPROCEDURE
///   FOR i <- 1 TO 3
///       CALL Inner()        // 等价于 Expr::Call 作为语句
///       OUTPUT i
///   NEXT i
/// 预期输出: 1 2 3  （函数里的 break 被吞掉，外层 FOR 不受影响）
#[test]
fn break_in_func_does_not_leak() {
	let ast = Stmt::Block {
		stmts: vec![
			Stmt::FuncDecl {
				name: "Inner".to_string(),
				params: vec![],
				return_type: Some(Type::Int),
				body: Box::new(Stmt::Block {
					stmts: vec![
						Stmt::Break { span: None },
						Stmt::Return {
							value: Some(Expr::Literal { value: RuntimeValue::Int(0), span: None }),
							span: None,
						},
					],
					span: None,
				}),
				span: None,
			},
			Stmt::For {
				var_name: "i".to_string(),
				start: Expr::Literal { value: RuntimeValue::Int(1), span: None },
				end: Expr::Literal { value: RuntimeValue::Int(3), span: None },
				body: Box::new(Stmt::Block {
					stmts: vec![
						// CALL Inner()  → 当成表达式语句
						Stmt::Expr {
							value: Expr::Call {
								name: "Inner".to_string(),
								args: vec![],
								span: None,
							},
							span: None,
						},
						Stmt::Print {
							value: vec![Expr::Get { name: "i".to_string(), span: None }],
							span: None,
						},
					],
					span: None,
				}),
				span: None,
			},
		],
		span: None,
	};

	print!("\n[break_in_func_does_not_leak] 预期: 1 2 3\n[break_in_func_does_not_leak] 实际: ");
	let mut interp = Interpreter::new();
	interp.execute(ast);
	println!();
}
