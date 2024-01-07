use crate::http::query_string::Value::Single;
use std::collections::HashMap;
#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}
#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>), // Vector는 동적으로 Heap에 원소를 할당한다
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value<'buf>> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..]
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        // 아래 3줄과 vec!는 같은 일을 한다
                        // let mut vec = Vec::new();
                        // vec.push(val);
                        // vec.push(prev_val);
                        // existing은 주소이기 때문에 그것이 가리키는 주소의 HashMap을 swap하기 위해서는 dereference가 필요
                        *existing = Value::Multiple(vec![prev_val, val]);
                    }
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
        }

        QueryString { data }
    }
}
