use caie_code_rs::{Expr, Interpreter, Op, RuntimeValue, Stmt, Type};

/// 多维数组测试（重点：验证递归 IndexAssign / Index）
///
/// CAIE 等价伪代码：
///   DECLARE M : ARRAY[1:2, 1:3] OF INTEGER
///
/// 在你的 AST 模型里，二维数组表达为"数组的数组"：
///   Type::Array( Type::Array(Int, 1, 3), 1, 2 )
/// 即 M 是长度 2 的外层数组，每个元素是长度 3 的内层数组。
///
///   M[1][1] <- 11
///   M[1][2] <- 12
///   M[2][3] <- 23
///   M[2][1] <- M[1][1] + M[1][2]   // 跨行读取后赋值，验证多维读+写
///
///   OUTPUT M[1][1], M[1][2], M[1][3]   // 第三列未赋值，应为 0
///   OUTPUT M[2][1], M[2][3]
#[test]
fn array_multi_dim() {
	// 内层类型：ARRAY[1:3] OF INTEGER
	let inner_type = Type::Array(Box::new(Type::Int), 1, 3);
	// 外层类型：ARRAY[1:2] OF (ARRAY[1:3] OF INTEGER)
	let outer_type = Type::Array(Box::new(inner_type), 1, 2);

	// 辅助：构造 M[i][j] 的读表达式
	let m_read = |i: i64, j: i64| -> Expr {
		Expr::Index {
			target: Box::new(Expr::Index {
				target: Box::new(Expr::Get { name: "M".to_string(), span: None }),
				index: Box::new(Expr::Literal { value: RuntimeValue::Int(i), span: None }),
				span: None,
			}),
			index: Box::new(Expr::Literal { value: RuntimeValue::Int(j), span: None }),
			span: None,
		}
	};
	// 辅助：构造 M[i][j] <- value 的语句
	let m_assign = |i: i64, j: i64, value: Expr| -> Stmt {
		Stmt::IndexAssign {
			target: Expr::Index {
				target: Box::new(Expr::Get { name: "M".to_string(), span: None }),
				index: Box::new(Expr::Literal { value: RuntimeValue::Int(i), span: None }),
				span: None,
			},
			index: Expr::Literal { value: RuntimeValue::Int(j), span: None },
			value,
			span: None,
		}
	};

	let mock_ast = Stmt::Block {
		stmts: vec![
			// DECLARE M : ARRAY[1:2, 1:3] OF INTEGER
			Stmt::VarDecl {
				name: "M".to_string(),
				var_type: outer_type,
				span: None,
			},
			// M[1][1] <- 11
			m_assign(1, 1, Expr::Literal { value: RuntimeValue::Int(11), span: None }),
			// M[1][2] <- 12
			m_assign(1, 2, Expr::Literal { value: RuntimeValue::Int(12), span: None }),
			// M[2][3] <- 23
			m_assign(2, 3, Expr::Literal { value: RuntimeValue::Int(23), span: None }),
			// M[2][1] <- M[1][1] + M[1][2]   (应为 11 + 12 = 23)
			m_assign(
				2, 1,
				Expr::Binary {
					left: Box::new(m_read(1, 1)),
					op: Op::Add,
					right: Box::new(m_read(1, 2)),
					span: None,
				},
			),
			// OUTPUT M[1][1], M[1][2], M[1][3]
			Stmt::Print {
				value: vec![m_read(1, 1), m_read(1, 2), m_read(1, 3)],
				span: None,
			},
			// OUTPUT M[2][1], M[2][3]
			Stmt::Print {
				value: vec![m_read(2, 1), m_read(2, 3)],
				span: None,
			},
		],
		span: None,
	};

	let mut interpreter = Interpreter::debug();
	interpreter.execute(mock_ast);

	// 预期输出:
	//   11 12 0       (M[1] 行：11, 12, 默认 0)
	//   23 23         (M[2][1]=11+12=23, M[2][3]=23)
}
