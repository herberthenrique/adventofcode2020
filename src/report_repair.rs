
//Solving first with brute force
pub fn find_error_part_one(expenses: Vec<i32>) -> i32 {
  for first in &expenses {
    for second in &expenses {
      let sum = first + second;
      if sum == 2020 {
        return first * second
      }
    }
  }
  return 0;
}


pub fn find_error_part_two(expenses: Vec<i32>) -> i32 {
  
  for first in &expenses {
    for second in &expenses {
      for third in &expenses {
        let sum = first + second + third;
        if sum == 2020 {
          return first * second * third
        }
      }
    }
  }

  return 0;
}