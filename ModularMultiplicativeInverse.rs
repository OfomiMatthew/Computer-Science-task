/* Eculidean algorithm basically is used to find
the greatest common divisor of two parameters, say
 'a' and 'm' and the multiplicative inverse is the number
 'x' such that '(a * x) % m = 1'

*/

/* The below function takes in two integer parameters
 'a' and 'm' as input and returns the modular multiplicative inverse
 of 'a' modulo 'm'.

 The conditional statement denotes that if the
 greatest common divisor of 'a' and 'm' is not 1,
 then there is no modular multiplicative inverse and the function returns zero
 

*/
fn modular_multiplicative_inverse(a: i32, m: i32) -> i32 {
  let mut m = m;
  let mut y = 0;
  let mut x = 1;

  if m == 1 {
      return 0;
  }

  let mut a = a;
  while a > 1 {
      let quotient = a / m;
      let t = m;
      m = a % m;
      a = t;
      t = y;
      y = x - quotient * y;
      x = t;
  }

  if x < 0 {
      x += m;
  }

  x
}
