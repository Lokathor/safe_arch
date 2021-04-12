use super::*;

#[test]
fn test_population_count_i32() {
  assert_eq!(population_count_i32(0), 0);
  assert_eq!(population_count_i32(0b1), 1);
  assert_eq!(population_count_i32(0b1001), 2);
}

#[test]
fn test_population_count_i64() {
  assert_eq!(population_count_i64(0), 0);
  assert_eq!(population_count_i64(0b1), 1);
  assert_eq!(population_count_i64(0b1001), 2);
}
