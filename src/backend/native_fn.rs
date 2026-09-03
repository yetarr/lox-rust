use std::rc::Rc;

use crate::backend::interpreter::callable::NativeFn;
use crate::frontend::lexer::token::LitVal;

pub fn clock() -> LitVal {
    LitVal::Callable(Rc::new(NativeFn {
        arity: 0,
        func: Box::new(|_interpreter, _args| {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64();
            Ok(LitVal::Number(now))
        }),
    }))
}
