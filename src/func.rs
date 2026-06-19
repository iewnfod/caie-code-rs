use crate::{CpcResult, Expr, Flow, Interpreter, RuntimeValue, Scope};

impl Interpreter {
    pub fn call_func(&mut self, name: String, args: Vec<Expr>) -> CpcResult<RuntimeValue> {
        let args = args.into_iter()
            .map(|arg| self.evaluate(arg))
            .collect::<CpcResult<Vec<_>>>()?;

        let func = self.get(name.clone());
        let func_obj = match func {
            Ok(RuntimeValue::Func(f)) => f,
            _ => return Err(crate::CpcError::Runtime {
                span: None,
                kind: crate::RuntimeErrorKind::UndefinedFunction(name),
            }),
        };
        let (body, closure, params, return_type) = {
            let f = func_obj.borrow();
            (
                f.body().clone(),
                f.closure().clone(),
                f.params().clone(),
                f.return_type().cloned(),
            )
        };

        if args.len() != params.len() {
            return Err(crate::CpcError::Runtime {
                span: None,
                kind: crate::RuntimeErrorKind::ArgCountMismatch {
                    expect: params.len(),
                    found: args.len(),
                },
            });
        }

        let child_scope = Scope::child(closure);
        for (param, arg_value) in params.into_iter().zip(args) {
            crate::scope::define(&child_scope, param.0, arg_value);
        }

        let saved_scope = std::mem::replace(&mut self.current_scope, child_scope);
        let flow = self.execute(body);
        self.current_scope = saved_scope;
        match flow {
            Ok(Flow::Return(val)) => Ok(val),
            Err(e) => Err(e),
            _ => {
                if return_type.is_some() {
                    Err(crate::CpcError::Runtime {
                        span: None,
                        kind: crate::RuntimeErrorKind::ReturnNotFound(name),
                    })
                } else {
                    Ok(RuntimeValue::Null)
                }
            },
        }
    }
}
