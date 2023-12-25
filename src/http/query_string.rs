use std::collections::HashMap;

pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>)  // Vector는 동적으로 Heap에 원소를 할당한다
}

impl <'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value<'buf>> {
        self.data.get(key)
    }
}