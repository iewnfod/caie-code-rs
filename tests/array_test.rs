use caie_code_rs::{Expr, Interpreter, ObjectAccess, Op, RuntimeValue, Stmt, TypeDefinition};

#[test]
fn array() {
	let mock_ast = Stmt::Block {
		stmts: vec![
			Stmt::VarDecl {
				name: "arr".to_string(),
				var_type: TypeDefinition::Array {
					element_type: Box::new(TypeDefinition::Primitive("INT".into())),
					start: 1,
					end: 5,
				},
				span: None,
			},
			Stmt::Assign {
				name: ObjectAccess::Direct {
					name: "arr".to_string(),
					span: None,
				},
				index: Some(Expr::Literal {
					value: RuntimeValue::Int(1),
					span: None,
				}),
				value: Expr::Binary {
					left: Box::new(Expr::Literal {value: RuntimeValue::Int(10), span: None}),
					op: Op::Add,
					right: Box::new(Expr::Literal {value: RuntimeValue::Int(5), span: None}),
					span: None,
				},
				span: None,
			},
			Stmt::Assign {
				name: ObjectAccess::Direct {
					name: "arr".to_string(),
					span: None,
				},
				index: Some(Expr::Literal {
					value: RuntimeValue::Int(2),
					span: None,
				}),
				value: Expr::Literal {
					value: RuntimeValue::Int(20),
					span: None
				},
				span: None,
			},
			Stmt::Print {
				value: vec![
					Expr::Literal {
						value: RuntimeValue::Str("arr[1]".to_string()), span: None,
					},
					Expr::Literal {
						value: RuntimeValue::Str("=".to_string()), span: None,
					},
					Expr::Get {
						name: ObjectAccess::Direct {
							name: "arr".to_string(),
							span: None,
						},
						index: Some(Box::new(Expr::Literal {
							value: RuntimeValue::Int(1),
							span: None,
						})),
						span: None,
					}
				],
				span: None,
			},
			Stmt::Print {
				value: vec![
					Expr::Literal {
						value: RuntimeValue::Str("arr".to_string()), span: None,
					},
					Expr::Literal {
						value: RuntimeValue::Str("=".to_string()), span: None,
					},
					Expr::Get {
						name: ObjectAccess::Direct {
							name: "arr".to_string(),
							span: None,
						},
						index: None,
						span: None,
					}
				],
				span: None,
			},
		],
		span: None,
	};

	let mut interpreter = Interpreter::debug();
	interpreter.execute(mock_ast);

	interpreter.print_environment();
}
