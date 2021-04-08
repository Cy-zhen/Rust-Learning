use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D{
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    } 
}

#[derive(Debug)]
struct Comlpex { real: f64, imag: f64, }

impl fmt::Display for Comlpex{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "real: {}, imag: {}", self.real, self.imag)
    }
}

fn main(){
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    //Compare structures:
    println!("Display: {}", minmax);
    //Display: (0, 14)
    println!("Debug: {:?}", minmax);
    //Debug: MinMax(0, 14)

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and  the small is {small}",
            small = small_range,
            big   = big_range
        );
    //The big range is (-300, 300) and  the small is (-3, 3)
    let point = Point2D{ x: 3.3, y: 7.2 };

    let complex = Comlpex{ real: 3.3, imag: 7.2 };

    println!("Compare points:");
    //Compare points:
    println!("Display: {}", point);
    //Display: x: 3.3, y: 7.2
    println!("Debug: {:?}", point);
    //Debug: Point2D { x: 3.3, y: 7.2 }
    println!("Display: {} + {} i", point.x, point.y);
    //Display: 3.3 + 7.2 i
    println!("Debug: {:?}", complex);
    //Debug: Comlpex { real: 3.3, imag: 7.2 }
}
