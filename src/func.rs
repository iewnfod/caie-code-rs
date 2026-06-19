use crate::{Expr, Interpreter, RuntimeValue, Scope};

impl Interpreter {
    pub fn call_func(&mut self, name: String, args: Vec<Expr>) -> RuntimeValue {
        let args = args.into_iter().map(|arg| self.evaluate(arg)).collect::<Vec<_>>();

        let func = self.get(name);
        let func_obj = match func {
            Some(RuntimeValue::Func(f)) => f,
            _ => return RuntimeValue::Null, // or handle error
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
            return RuntimeValue::Null; // or handle error
        }

        let child_scope = Scope::child(closure);
        for (param, arg) in params.into_iter().zip(args) {
            if let Some(arg_val) = arg {
                crate::scope::define(&child_scope, param.0, arg_val);
            } else {
                return RuntimeValue::Null; // or handle error
            }
        }

        let saved_scope = std::mem::replace(&mut self.current_scope, child_scope);
        let flow = self.execute(body);
        self.current_scope = saved_scope;
        match flow {
            crate::Flow::Return(val) => val,
            _ => RuntimeValue::Null, // or handle error
        }
    }
}
