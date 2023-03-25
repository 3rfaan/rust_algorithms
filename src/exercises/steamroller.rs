// Flatten a nested array. You must account for varying levels of nesting.

#[allow(dead_code)]
#[allow(unused_variables)]
enum Value {
    Number(i32),
    Array(Vec<Value>),
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn steamroll_array(arr: &[Value]) -> Vec<i32> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let arr = [
            Value::Number(1),
            Value::Array(vec![Value::Number(2)]),
            Value::Array(vec![
                Value::Number(3),
                Value::Array(vec![Value::Array(vec![Value::Number(4)])]),
            ]),
        ];

        assert_eq!(steamroll_array(&arr), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test2() {
        let arr = [
            Value::Number(1),
            Value::Array(vec![]),
            Value::Array(vec![
                Value::Number(3),
                Value::Array(vec![Value::Array(vec![Value::Number(4)])]),
            ]),
        ];

        assert_eq!(steamroll_array(&arr), vec![1, 3, 4]);
    }
}
