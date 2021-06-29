use std::convert::TryFrom;

struct Action {
    subject: String,
    object: String,
    verb: String,
}

impl Action {
    fn new() -> Self {
        todo!()
    }
}

impl TryFrom<&str> for Action {
    type Error = &'static str;

    fn try_from(_input: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}
