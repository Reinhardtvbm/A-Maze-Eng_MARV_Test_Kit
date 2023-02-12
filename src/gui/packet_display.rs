use std::collections::VecDeque;

pub struct LabelList(VecDeque<String>);

impl LabelList {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push(&mut self, string: &str) {
        self.0.push_front(String::from(string));

        if self.0.len() > 20 {
            self.0.pop_back();
        }
    }

    pub fn to_vec(&self) -> Vec<String> {
        self.0.clone().into()
    }
}
