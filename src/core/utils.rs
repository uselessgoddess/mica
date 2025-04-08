#[must_use]
pub fn type_name<T: ?Sized>() -> &'static str {
  std::any::type_name::<T>().split("::").last().unwrap()
}

#[test]
fn test_type_name() {
  struct Custom;

  assert_eq!(type_name::<i32>(), "i32");
  assert_eq!(type_name::<String>(), "String");
  assert_eq!(type_name::<Custom>(), "Custom");
}
