use caie_code_rs::{Expr, Interpreter, ObjectAccess, Op, RuntimeValue, Stmt, TypeDefinition};

#[test]
fn test_assignment() {
	// 手动创建 AST
	let mock_ast = Stmt::Block {
		stmts: vec![
			Stmt::VarDecl {
				name: "X".to_string(),
				var_type: TypeDefinition::Primitive("INT".into()),
				span: None,
			},
			Stmt::Assign {
				name: ObjectAccess::Direct {
					name: "X".to_string(),
					span: None,
				},
				index: None,
				value: Expr::Binary {
					left: Box::new(Expr::Literal {value: RuntimeValue::Int(10), span: None}),
					op: Op::Add,
					right: Box::new(Expr::Literal {value: RuntimeValue::Int(5), span: None}),
					span: None,
				},
				span: None,
			}
		],
		span: None,
	};

	// 执行内核逻辑
	let mut interpreter = Interpreter::new();
	interpreter.execute(mock_ast);

	println!("Environment after execution: {:?}", interpreter.environment);
	let value = interpreter.environment.get("X".to_string()).unwrap().0.borrow().get_var_value().unwrap();
	println!("Value of X: {:?}", value);

	// 验证结果
	assert_eq!(value, RuntimeValue::Int(15));
}
