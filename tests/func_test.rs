use caie_code_rs::{Expr, Interpreter, Op, RuntimeValue, Stmt, Type};

/// 测试 1：基本函数定义与调用（验证 FuncDecl 执行 + Expr::Call 求值）
///   FUNCTION Add(A : INTEGER, B : INTEGER) RETURNS INTEGER
///       RETURN A + B
///   ENDFUNCTION
///   OUTPUT Add(3, 4)        // 预期 7
#[test]
fn func_basic_call() {
	let ast = Stmt::Block {
		stmts: vec![
			Stmt::FuncDecl {
				name: "Add".to_string(),
				params: vec![
					("A".to_string(), Type::Int),
					("B".to_string(), Type::Int),
				],
				return_type: Some(Type::Int),
				body: Box::new(Stmt::Block {
					stmts: vec![
						Stmt::Return {
							value: Some(Expr::Binary {
								left: Box::new(Expr::Get { name: "A".to_string(), span: None }),
								op: Op::Add,
								right: Box::new(Expr::Get { name: "B".to_string(), span: None }),
								span: None,
							}),
							span: None,
						},
					],
					span: None,
				}),
				span: None,
			},
			Stmt::Print {
				value: vec![Expr::Call {
					name: "Add".to_string(),
					args: vec![
						Expr::Literal { value: RuntimeValue::Int(3), span: None },
						Expr::Literal { value: RuntimeValue::Int(4), span: None },
					],
					span: None,
				}],
				span: None,
			},
		],
		span: None,
	};

	print!("\n[func_basic_call] 预期: 7\n[func_basic_call] 实际: ");
	let mut interp = Interpreter::new();
	interp.execute(ast);
	println!();
}

/// 测试 2：IF 里的 RETURN（验证 execute 的 If 分支正确透传 Flow）
///   FUNCTION F() RETURNS INTEGER
///       IF TRUE THEN
///           RETURN 42
///       ENDIF
///       RETURN 0
///   ENDFUNCTION
///   OUTPUT F()            // 预期 42（如果 execute 丢弃 Flow 会输出 0 或 Null）
#[test]
fn func_return_inside_if() {
	let ast = Stmt::Block {
		stmts: vec![
			Stmt::FuncDecl {
				name: "F".to_string(),
				params: vec![],
				return_type: Some(Type::Int),
				body: Box::new(Stmt::Block {
					stmts: vec![
						Stmt::If {
							condition: Expr::Literal { value: RuntimeValue::Bool(true), span: None },
							true_body: Box::new(Stmt::Return {
								value: Some(Expr::Literal { value: RuntimeValue::Int(42), span: None }),
								span: None,
							}),
							false_body: None,
							span: None,
						},
						Stmt::Return {
							value: Some(Expr::Literal { value: RuntimeValue::Int(0), span: None }),
							span: None,
						},
					],
					span: None,
				}),
				span: None,
			},
			Stmt::Print {
				value: vec![Expr::Call {
					name: "F".to_string(),
					args: vec![],
					span: None,
				}],
				span: None,
			},
		],
		span: None,
	};

	print!("\n[func_return_inside_if] 预期: 42\n[func_return_inside_if] 实际: ");
	let mut interp = Interpreter::new();
	interp.execute(ast);
	println!();
}

/// 测试 3：递归（验证函数能调自己 = 词法作用域 + 定义点可见）
///   FUNCTION Fact(N : INTEGER) RETURNS INTEGER
///       IF N <= 1 THEN
///           RETURN 1
///       ENDIF
///       RETURN N * Fact(N - 1)
///   ENDFUNCTION
///   OUTPUT Fact(5)         // 预期 120
#[test]
fn func_recursion() {
	let ast = Stmt::Block {
		stmts: vec![
			Stmt::FuncDecl {
				name: "Fact".to_string(),
				params: vec![("N".to_string(), Type::Int)],
				return_type: Some(Type::Int),
				body: Box::new(Stmt::Block {
					stmts: vec![
						Stmt::If {
							condition: Expr::Binary {
								left: Box::new(Expr::Get { name: "N".to_string(), span: None }),
								op: Op::Lte,
								right: Box::new(Expr::Literal { value: RuntimeValue::Int(1), span: None }),
								span: None,
							},
							true_body: Box::new(Stmt::Return {
								value: Some(Expr::Literal { value: RuntimeValue::Int(1), span: None }),
								span: None,
							}),
							false_body: None,
							span: None,
						},
						Stmt::Return {
							value: Some(Expr::Binary {
								left: Box::new(Expr::Get { name: "N".to_string(), span: None }),
								op: Op::Mul,
								right: Box::new(Expr::Call {
									name: "Fact".to_string(),
									args: vec![Expr::Binary {
										left: Box::new(Expr::Get { name: "N".to_string(), span: None }),
										op: Op::Sub,
										right: Box::new(Expr::Literal { value: RuntimeValue::Int(1), span: None }),
										span: None,
									}],
									span: None,
								}),
								span: None,
							}),
							span: None,
						},
					],
					span: None,
				}),
				span: None,
			},
			Stmt::Print {
				value: vec![Expr::Call {
					name: "Fact".to_string(),
					args: vec![Expr::Literal { value: RuntimeValue::Int(5), span: None }],
					span: None,
				}],
				span: None,
			},
		],
		span: None,
	};

	print!("\n[func_recursion] 预期: 120\n[func_recursion] 实际: ");
	let mut interp = Interpreter::new();
	interp.execute(ast);
	println!();
}

/// 测试 4：函数能访问外层变量（验证词法作用域 / 闭包）
///   DECLARE Base : INTEGER
///   Base <- 100
///   FUNCTION AddBase(X : INTEGER) RETURNS INTEGER
///       RETURN Base + X       // Base 来自定义点的外层 scope
///   ENDFUNCTION
///   OUTPUT AddBase(23)     // 预期 123
#[test]
fn func_closure_capture() {
	let ast = Stmt::Block {
		stmts: vec![
			Stmt::VarDecl { name: "Base".to_string(), var_type: Type::Int, span: None },
			Stmt::Assign {
				name: "Base".to_string(),
				value: Expr::Literal { value: RuntimeValue::Int(100), span: None },
				span: None,
			},
			Stmt::FuncDecl {
				name: "AddBase".to_string(),
				params: vec![("X".to_string(), Type::Int)],
				return_type: Some(Type::Int),
				body: Box::new(Stmt::Block {
					stmts: vec![
						Stmt::Return {
							value: Some(Expr::Binary {
								left: Box::new(Expr::Get { name: "Base".to_string(), span: None }),
								op: Op::Add,
								right: Box::new(Expr::Get { name: "X".to_string(), span: None }),
								span: None,
							}),
							span: None,
						},
					],
					span: None,
				}),
				span: None,
			},
			Stmt::Print {
				value: vec![Expr::Call {
					name: "AddBase".to_string(),
					args: vec![Expr::Literal { value: RuntimeValue::Int(23), span: None }],
					span: None,
				}],
				span: None,
			},
		],
		span: None,
	};

	print!("\n[func_closure_capture] 预期: 123\n[func_closure_capture] 实际: ");
	let mut interp = Interpreter::new();
	interp.execute(ast);
	println!();
}
