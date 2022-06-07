// `RawParameters` is the vehicle used by the `Op`erator factory in `Op::op(...)`,
// to ferry args around from the invocator into the constructor of the individual
// `InnerOp`s. The `InnrOp`constructor typically interprets the contents of
// `RawParameters`, and converts it into a more runtime friendly instance of
// `ParsedParameters`.

use super::*;

#[derive(Debug, Default)]
pub struct RawParameters {
    pub invocation: String,
    pub definition: String,
    pub globals: BTreeMap<String, String>,
    recursion_level: usize,
}

impl RawParameters {
    pub fn new(invocation: &str, globals: &BTreeMap<String, String>) -> RawParameters {
        let recursion_level = 0;
        let globals = globals.clone();
        let invocation = invocation.to_string();
        let definition = invocation.clone();

        // If it is a macro invocation, the `next()` method is called
        // to do the parameter handling
        if super::is_resource_name(&invocation) {
            let definition = "".to_string();
            let previous = RawParameters {
                invocation,
                definition,
                globals,
                recursion_level,
            };
            return previous.next(&previous.invocation);
        }

        // Not a macro? Then it is either a pipeline, a built-in or
        // a user defined op, and we can just carry on
        RawParameters {
            invocation,
            definition,
            globals,
            recursion_level,
        }
    }

    // If the next step is a macro (i.e. potentially an embedded pipeline), we
    // get the arguments from the invocation and bring them into the globals.
    // Otherwise, we just copy the globals from the previous step, and
    // update the recursion counter.
    pub fn next(&self, definition: &str) -> RawParameters {
        let mut recursion_level = self.recursion_level + 1;
        let mut globals = self.globals.clone();
        if super::is_resource_name(definition) {
            globals.remove("name");
            globals.extend(super::split_into_parameters(definition).into_iter());
            globals.remove("inv");
            recursion_level += 1;
        }
        let invocation = self.invocation.clone();
        let definition = definition.trim().to_string();
        RawParameters {
            invocation,
            definition,
            globals,
            recursion_level,
        }
    }

    pub fn nesting_too_deep(&self) -> bool {
        self.recursion_level > 100
    }
}

// ----- T E S T S ---------------------------------------------------------------------

// RawParameters gets its test coverage from the tests in `op/mod.rs`
