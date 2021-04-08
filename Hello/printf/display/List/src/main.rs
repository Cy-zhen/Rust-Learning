use std::fmt;
struct List(Vec<i32>);
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f,"[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ",")?;}
            //write!("v[{}]: ", count, "{}", v)?;//报错1  
            write!(f, "v[{}]: {} ", count, v)?;
            //write!(f, "v[{t}]: ", t = count, "{}",  v)?;
           // write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}
fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{} ", v);
    //[v[0]: 1 ,v[1]: 2 ,v[2]: 3 ] 
}




//error1
/*    /*Compiling playground v0.0.1 (/playground)
error: format argument must be a string literal
  --> src/main.rs:18:32
   |
18 |             write!("v[{}]: ",  count, "{}", v)?;
   |                                ^^^^^
   |
help: you might be missing a string literal to format with
   |
18 |             write!("v[{}]: ",  "{} {} {}", count, "{}", v)?;
   |                                ^^^^^^^^^^^

error[E0599]: no method named `write_fmt` found for reference `&'static str` in the current scope
  --> src/main.rs:18:13
   |
18 |             write!("v[{}]: ",  count, "{}", v)?;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&'static str`
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
error: could not compile `playground`

To learn more, run the command again with --verbose. */ */