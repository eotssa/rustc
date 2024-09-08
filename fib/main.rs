fn fib(n: u32) -> u32 {
  if n < 2 { // Base Case
     return n
  } else { // Recursive case
     return fib(n - 2) + fib(n - 1)
  }
}

fn main() {
  let n = 20;
  println!("fib({n}) = {}", fib(n));
}
