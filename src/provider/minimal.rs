use super::*;
use std::path::PathBuf;

// ----- T H E   M I N I M A L   P R O V I D E R ---------------------------------------

/// A minimalistic context provider, supporting only built in and run-time defined operators.
/// Usually sufficient for cartographic uses, and for internal test authoring.
#[derive(Debug, Default)]
pub struct Minimal {
    /// Constructors for user defined operators
    constructors: BTreeMap<String, OpConstructor>,
    /// User defined resources (macros)
    resources: BTreeMap<String, String>,
    /// Instantiations of operators
    operators: BTreeMap<Uuid, Op>,
}

impl Provider for Minimal {
    fn op(&mut self, definition: &str) -> Result<Uuid, Error> {
        let op = Op::new(definition, self)?;
        let id = op.id;
        self.operators.insert(id, op);
        assert!(self.operators.contains_key(&id));
        Ok(id)
    }
    // Op::new("foo:baz", &prv),
    // prv.op("foo.baz"),

    fn apply(
        &self,
        op: Uuid,
        direction: Direction,
        operands: &mut [Coord],
    ) -> Result<usize, Error> {
        const BAD_ID_MESSAGE: Error = Error::General("Minimal: Unknown operator id");
        let op = self.operators.get(&op).ok_or(BAD_ID_MESSAGE)?;
        op.apply(self, operands, direction)
    }

    fn globals(&self) -> BTreeMap<String, String> {
        BTreeMap::from([("ellps".to_string(), "GRS80".to_string())])
    }

    fn register_op(&mut self, name: &str, constructor: OpConstructor) {
        self.constructors.insert(String::from(name), constructor);
    }

    fn get_op(&self, name: &str) -> Result<OpConstructor, Error> {
        if let Some(result) = self.constructors.get(name) {
            return Ok(OpConstructor(result.0));
        }

        Err(Error::NotFound(
            name.to_string(),
            ": User defined constructor".to_string(),
        ))
    }

    fn register_resource(&mut self, name: &str, definition: &str) {
        self.resources
            .insert(String::from(name), String::from(definition));
    }

    fn get_resource(&self, name: &str) -> Result<String, Error> {
        if let Some(result) = self.resources.get(name) {
            return Ok(result.to_string());
        }

        Err(Error::NotFound(
            name.to_string(),
            ": User defined resource".to_string(),
        ))
    }

    fn access(&self, name: &str) -> Result<Vec<u8>, Error> {
        let mut path = PathBuf::from("geodesy");
        path.push(&name);
        Ok(std::fs::read(path)?)
    }
}
