fn main() {
   let t:f64 = 900000.0;
   let m:f64 = 1500000.0;
   let h:f64 = 2250000.0;
   let d:f64 = 8550000.0;
   let a:f64 = 250000.0;

   // sum and average 
   let s = t + m + h + d + a;
   println!("Sum is {}", s);
   let v = (t + m + h + d + a) / 5.0;
   println!("Average is {}", v);

}