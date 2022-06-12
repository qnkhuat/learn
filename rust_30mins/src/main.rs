fn main() {

  // basic declaration and priting
  let x: i32 = 12;
  let x = x + 3;
  println!("X is: {x}, y is {y}", y = 3, x = x);

  let number: f64 = 1.0;
  let width: usize = 6;
  println!("Print number with padding {number:>width$}");


  // tuples
  let pair = ('a', 13);
  println!("First element of pair: {}", pair.0);


  let pair_with_types: (char, i32) = ('b', 13);
  println!("First element of pair: {}", pair_with_types.0);

  // destructuring
  let (first_elem, _second_elem) = ('c', 12);
  println!("Destructring: {}", first_elem);

  let (left, right) = "ngockq".split_at(3);
  println!("Left: {}, right: {}", left, right);

  // statements chaining
  let x = vec![1, 2, 3, 4, 5]
    .iter()
    .map(|x| x + 3)
    .fold(0, |x, y| x + y);
  println!("Statements chaining for now: {}", x);

  // define a function
  fn greet() {
    println!("Hello world?");
  }
  greet();

  fn add(x: i32, y: i32) -> i32 {
    x + y
  }

  println!("ADDING? : 1 + 2 = {}", add(1, 2));

  #[allow(dead_code)]
  fn scoping() {
    let x = "OUTMOST";
    {
      let x = "INNER";
      println!("MEH? : {}", x);
    }

    println!("HEH? : {}", x);
  }

  // block is an expression
  let sum_of_1_and_2 = {
    let x = 1;
    let y = 2;
    x + y // the same as return x + y ;
  };
  println!("block is an expression: {}", sum_of_1_and_2);


  // if else
  if true {
    println!("TRUE");
  } else {
    println!("FALSE");
  }

  // match
  let lucky = true;
  println!("LUCKY?: {}", match lucky {
    true => "YES",
    false => "NO"
  });


  // accessing ns
  let least = std::cmp::min(3, 8);
  println!("min of 3 and 8?: {}", least);


  // importng
  use std::cmp::min;
  let least = min(3, 8);
  println!("no more direct access: min of 3 and 8?: {}", least);

  use std::cmp::{max, max_by};
  println!("Bulk importing: {}", max(1, 2));
  println!("Bulk importing: {}", max_by(1, 2, |_x, _y| std::cmp::Ordering::Less));

  // who cares?
  use std::cmp::*;

  let x_count = "muay_bien".len();

  println!("x_count: {}", x_count);


  // vector
  let v: Vec<i32> = Vec::new();

  struct Point {
    x: f64,
    y: f64,
  }

  let p1 = Point{x: 1.0, y: 2.0 };
  let p2 = Point{..p1};
  let Point {x, y} = p1;
  let Point {x, ..} = p2;

  //println!("p2: {}", p2);
  //
  struct Number {
    odd: bool,
    value: i32,
  }

  fn print_number(n: Number) {
    if let Number {odd: true, value} = n {
      println!("This is an odd number: {}", value);
    } else if let Number {odd: false, value} = n {
      println!("This is an even number: {}", value);
    } else {
      println!("What else it could be?");
    }
  }
  let n1 = Number{odd: true, value: 1};
  let n2 = Number{odd: false, value: 2};
  print_number(n1);


  fn print_number_with_match(n: Number) {
    match n {
      Number { odd: true, value } => println!("Odd number; {}", value),
      Number { odd: false, value } => println!("Even number; {}", value),
      _ => println!("LOL"),
    }
  }

  print_number_with_match(n2);

  struct Number2 {
    odd: bool,
    value: i32,
  }

  impl Number2 {
    fn is_strictly_positive(self) -> bool {
      self.value > 0
    }
  }

  let n23 = Number2{odd: true, value: 3};

  println!("Is it positive?: {}", n23.is_strictly_positive());

  // mutable variable
  let mut n = Number {
    odd: true,
    value: 17,
  };

  n.value = 19; // all good


  // trait
  trait Signed {
    fn is_strictly_positive(self) -> bool;
  }

  impl Signed for i32 {
    fn is_strictly_positive(self) -> bool {
      self > 0
    }
  }

  let n: i32 = -3;

  println!("Is this i32 with value {} positive?: {}", n, n.is_strictly_positive());



  impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "({}, {})", self.value, self.odd)
    }
  }

  impl std::ops::Neg for Number {
    type Output = Number;

    fn neg(self) -> Number {
      Number{
        value: -self.value,
        odd: self.odd,
      }
    }
  }

  let n = Number{ odd: true, value: 123 };
  let m = -n; // this works because we implement Neg trait
  println!("negative of n is: {}", m);


  // working with reference

  fn print_nber(n: Number) {
    println!("({}, {})", n.value, n.odd);
  }
  let n1 = Number{ odd: true, value: 123 };
  print_nber(n1);
  //print_nber(n1); this won't work because Number couldn't be copied yet
  fn print_nber2(n: &Number) {
    println!("({}, {})", n.value, n.odd);
  }
  let n2 = Number{ odd: true, value: 123 };
  print_nber2(&n2); // n2 is borrowed for the tiem of call;
  print_nber2(&n2);

  fn increase(n: &mut  Number) {
    n.value += 1;
    n.odd = !n.odd;
  }
  let mut n3 = Number{ odd: true, value: 1 };
  increase(&mut n3);


  impl std::clone::Clone for Number {
    fn clone(&self) -> Self {
      Self { ..*self }
    }
  }

  let n = Number{odd: true, value: 1};
  let mut m = n.clone();
  m.value += 200;


  // automatically implement these 2 traits
  #[derive(Clone, Copy)]
  struct DivideNumber {
    odd: bool,
    value: i32,
  }

  let dn = DivideNumber{odd: true, value: 3};
  dn.odd;
  dn.value;

  fn random_generic_fn<L, R>(left: L, right: R) {
  }


  // generic function with constraint to display
  fn generic_print<T: std::fmt::Display>(arg: T) {
    println!("ARG: {}", arg);
  }
  generic_print(3);


  fn generic_compare<T>(left: T, right:T)
    where
      T: std::fmt::Debug + std::fmt::Display,
    {
      println!("Left: {}, right: {}", left, right);
    }

  generic_compare(3, 4);

  // generic struct
  struct Pair<T> {
    a: T,
    b: T,
  }

  fn print_type_name<T>(_val: &T) {
    println!("{}", std::any::type_name::<T>());
  }

  let p1 = Pair{ a: 3, b: 9 };
  let p2 = Pair{ a: true, b: false };
  print_type_name(&p1);
  print_type_name(&p2);


  // macro, most fn ending with ! is a macro
  println!("{}", "Hello world!");
  // -> io::stdout().lock().write_all(b"Hello world!\n").unwrap();
  let v1 = vec![1, 2, 3]; // macro
  let v2 = vec![true, true, false]; // macro

  // Option is an enum
  let o1: Option<i32> = Some(128);
  o1.unwrap(); // fine and returns 128
  let o1: Option<i32> = None;
  o1.unwrap(); // panics


  // Result is also an enum

  // life time
}
