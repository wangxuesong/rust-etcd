pub struct ComparisonConditions<'a> {
    pub modified_index: Option<u64>,
    pub value: Option<&'a str>,
}

impl<'a> ComparisonConditions<'a> {
    pub fn is_empty(&self) -> bool {
        self.modified_index.is_none() && self.value.is_none()
    }
}
