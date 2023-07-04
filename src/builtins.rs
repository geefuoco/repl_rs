use crate::object::ErrorObject;
use crate::object::Integer;
use crate::object::Object;
use crate::object::Objects;

pub struct BuiltinFunctions {}

impl BuiltinFunctions {
    pub fn len(args: &[Objects]) -> Objects {
        if args.len() != 1 {
            return Objects::Error(ErrorObject::new(format!(
                "expected 1 argument but received {}",
                args.len()
            )));
        }
        let obj = &args[0];
        match obj {
            Objects::String(s) => Objects::Integer(Integer::new(s.value().len() as isize)),
            _ => {
                return Objects::Error(ErrorObject::new(format!(
                    "argument to 'len' not supported, got {}",
                    obj.obj_type()
                )));
            }
        }
    }
}
