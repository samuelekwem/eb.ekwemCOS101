fn main() {
   let p:f64 = 510000.0;
   let r:f64 = 5.0;
   let t:f64 = 3.0;

   // amount
   let a = p * (1.0 - (r / 100.0)) * t;
   println!("Amount is {}", a);
   	
}