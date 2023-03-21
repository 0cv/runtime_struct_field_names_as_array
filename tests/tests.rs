#![allow(dead_code)]

use runtime_struct_field_names_as_array::FieldNamesAsArray;

#[derive(FieldNamesAsArray)]
struct Parent {
  parent1: Option<String>,
  parent2: String,
}

#[test]
fn test_with_parents() {
  #[derive(FieldNamesAsArray)]
  struct Test {
    test1: String,
    test2: i64,
    test3: String,
    #[field_names_as_array(flatten)]
    test4: Parent,
    #[field_names_as_array(flatten)]
    test5: Option<Parent>,
  }

  assert_eq!(
    Test::field_names_as_array(),
    [
      "test1",
      "test2",
      "test3",
      "test4.parent1",
      "test4.parent2",
      "test5.parent1",
      "test5.parent2"
    ]
  );
}

#[test]
fn test_without_parents() {
  #[derive(FieldNamesAsArray)]
  struct Test {
    test1: String,
    test2: i64,
    test3: String,
    test4: Parent,
    test5: Option<Parent>,
  }

  assert_eq!(
    Test::field_names_as_array(),
    ["test1", "test2", "test3", "test4", "test5"]
  );
}

#[test]
fn test_mixed() {
  #[derive(FieldNamesAsArray)]
  struct Test {
    test1: String,
    test2: i64,
    test3: String,
    test4: Parent,
    #[field_names_as_array(flatten)]
    test5: Option<Parent>,
  }

  assert_eq!(
    Test::field_names_as_array(),
    [
      "test1",
      "test2",
      "test3",
      "test4",
      "test5.parent1",
      "test5.parent2",
    ]
  );
}
