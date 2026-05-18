use caie_code_rs::{Expr, Interpreter, ObjectAccess, RuntimeValue, Stmt, TypeDefinition};

#[test]
fn if_stmt() {
	// 手动创建 AST
	let mock_ast = Stmt::Block {
		stmts: vec![
			Stmt::FuncDecl {
				name: "hello".to_string(),
				params: vec![("name".to_string(), TypeDefinition::Primitive("STRING".into()))],
				body: Box::new(Stmt::Block { stmts: vec![
					Stmt::Print {
						value: vec![
							Expr::Literal { value: RuntimeValue::Str("Hello, World!".to_string()), span: None },
						],
						span: None
					},
					Stmt::Print {
						value: vec![
							Expr::Literal { value: RuntimeValue::Str("Hello".to_string()), span: None },
							Expr::Get { name: ObjectAccess::Direct { name: "name".to_string(), span: None }, index: None, span: None }
						],
						span: None
					},
				], span: None }),
				span: None
			},
			Stmt::Expr {
				value: Expr::Call {
					name: ObjectAccess::Direct { name: "hello".to_string(), span: None },
					args: vec![Expr::Literal { value: RuntimeValue::Str("Iewnfod".to_string()), span: None }],
					span: None
				},
				span: None
			}
		],
		span: None
	};

	// 执行内核逻辑
	let mut interpreter = Interpreter::debug();
	interpreter.execute(mock_ast);

	interpreter.print_environment();
}
