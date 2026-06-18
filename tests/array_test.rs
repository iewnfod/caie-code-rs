use caie_code_rs::{Expr, Interpreter, Op, RuntimeValue, Stmt, Type};

/// 一维数组测试
/// CAIE 等价伪代码：
///   DECLARE A : ARRAY[1:5] OF INTEGER
///   A[1] <- 10
///   A[2] <- A[1] + 5      // 读 A[1] 参与运算
///   OUTPUT A[1], A[2], A[3]   // A[3] 未赋值，应为默认值 0
#[test]
fn array_basic() {
	let mock_ast = Stmt::Block {
		stmts: vec![
			// DECLARE A : ARRAY[1:5] OF INTEGER
			Stmt::VarDecl {
				name: "A".to_string(),
				var_type: Type::Array(Box::new(Type::Int), 1, 5),
				span: None,
			},
			// A[1] <- 10
			Stmt::IndexAssign {
				target: Expr::Get { name: "A".to_string(), span: None },
				index: Expr::Literal { value: RuntimeValue::Int(1), span: None },
				value: Expr::Literal { value: RuntimeValue::Int(10), span: None },
				span: None,
			},
			// A[2] <- A[1] + 5
			Stmt::IndexAssign {
				target: Expr::Get { name: "A".to_string(), span: None },
				index: Expr::Literal { value: RuntimeValue::Int(2), span: None },
				value: Expr::Binary {
					left: Box::new(Expr::Index {
						target: Box::new(Expr::Get { name: "A".to_string(), span: None }),
						index: Box::new(Expr::Literal { value: RuntimeValue::Int(1), span: None }),
						span: None,
					}),
					op: Op::Add,
					right: Box::new(Expr::Literal { value: RuntimeValue::Int(5), span: None }),
					span: None,
				},
				span: None,
			},
			// OUTPUT A[1], A[2], A[3]
			Stmt::Print {
				value: vec![
					Expr::Index {
						target: Box::new(Expr::Get { name: "A".to_string(), span: None }),
						index: Box::new(Expr::Literal { value: RuntimeValue::Int(1), span: None }),
						span: None,
					},
					Expr::Index {
						target: Box::new(Expr::Get { name: "A".to_string(), span: None }),
						index: Box::new(Expr::Literal { value: RuntimeValue::Int(2), span: None }),
						span: None,
					},
					Expr::Index {
						target: Box::new(Expr::Get { name: "A".to_string(), span: None }),
						index: Box::new(Expr::Literal { value: RuntimeValue::Int(3), span: None }),
						span: None,
					},
				],
				span: None,
			},
		],
		span: None,
	};

	let mut interpreter = Interpreter::debug();
	interpreter.execute(mock_ast);

	// 预期输出: 10 15 0
	// 其中 A[3] 是声明后未赋值的默认值 0
}
