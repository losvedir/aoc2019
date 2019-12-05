pub fn puzzle1() {
  let mut valid = 0;

  for i0 in 0..10 {
    for i1 in 0..10 {
      for i2 in 0..10 {
        for i3 in 0..10 {
          for i4 in 0..10 {
            for i5 in 0..10 {
              if valid_password1(i5, i4, i3, i2, i1, i0) {
                valid += 1;
              }
            }
          }
        }
      }
    }
  }

  println!("day4 puzzle1: {}", valid);
}

pub fn puzzle2() {
  let mut valid = 0;

  for i0 in 0..10 {
    for i1 in 0..10 {
      for i2 in 0..10 {
        for i3 in 0..10 {
          for i4 in 0..10 {
            for i5 in 0..10 {
              if valid_password2(i5, i4, i3, i2, i1, i0) {
                valid += 1;
              }
            }
          }
        }
      }
    }
  }

  println!("day4 puzzle2: {}", valid);
}

fn valid_password1(i5: u32, i4: u32, i3: u32, i2: u32, i1: u32, i0: u32) -> bool {
  if i5 <= i4 && i4 <= i3 && i3 <= i2 && i2 <= i1 && i1 <= i0 {
    if i5 == i4 || i4 == i3 || i3 == i2 || i2 == i1 || i1 == i0 {
      let value = i5 * 100000 + i4 * 10000 + i3 * 1000 + i2 * 100 + i1 * 10 + i0;

      if value >= 353096 && value <= 843212 {
        return true;
      }
    }
  }

  return false;
}

fn valid_password2(i5: u32, i4: u32, i3: u32, i2: u32, i1: u32, i0: u32) -> bool {
  if i5 <= i4 && i4 <= i3 && i3 <= i2 && i2 <= i1 && i1 <= i0 {
    if (i5 == i4 && i4 != i3)
      || (i4 == i3 && i4 != i5 && i3 != i2)
      || (i3 == i2 && i3 != i4 && i2 != i1)
      || (i2 == i1 && i2 != i3 && i1 != i0)
      || (i1 == i0 && i1 != i2)
    {
      let value = i5 * 100000 + i4 * 10000 + i3 * 1000 + i2 * 100 + i1 * 10 + i0;

      if value >= 353096 && value <= 843212 {
        return true;
      }
    }
  }

  return false;
}
