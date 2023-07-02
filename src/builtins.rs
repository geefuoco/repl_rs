use crate::object::BuiltinWrapper;
use crate::object::ErrorObject;
use crate::object::Integer;
use crate::object::Null;
use crate::object::Object;
use crate::object::Objects;

const BUILTIN_FUNCTIONS_COUNT: usize = 2;

pub struct BuiltinFunctions {
    function_list: [BuiltinWrapper; BUILTIN_FUNCTIONS_COUNT],
}

pub enum BuiltinFunctionNames {
    Len = 0,
    Drop = 1,
}

impl BuiltinFunctions {
    pub const fn new() -> Self {
        let v = [BuiltinWrapper::new(BuiltinFunctions::len), BuiltinWrapper::new(BuiltinFunctions::drop)];
        Self { function_list: v }
    }

    pub fn get(&self, key: BuiltinFunctionNames) -> &BuiltinWrapper {
        match key {
            BuiltinFunctionNames::Len => &self.function_list[BuiltinFunctionNames::Len as usize],
            _ => panic!("Tried to query for a function that was not implemented"),
        }
    }

    fn len(args: &[Objects]) -> Objects {
        if args.len() != 1 {
            return Objects::Error(ErrorObject::new(format!(
                "expected 1 argument but received {}",
                args.len()
            )));
        }
        let obj = &args[0];
        match obj {
            Objects::String(s) => Objects::Integer(Integer::new(
                s.value().len() as isize,
            )),
            _ => {
                return Objects::Error(ErrorObject::new(format!(
                    "argument to 'len' not supported, got {}",
                    obj.obj_type()
                )));
            }
        }
    }

    fn drop(args: &[Objects]) -> Objects {
        if args.len() != 1 {
            return Objects::Error(ErrorObject::new(format!(
                "expected 1 argument but received {}",
                args.len()
            )));
        }
        //drop logic
        return Objects::Null(Null{ });
    }
}
