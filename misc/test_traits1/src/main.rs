fn main() {
    let x = Point2D {
        x: 10_i32,
        y: 20_i32,
    };
    let t = Point1D { x: 111_i32 };
    //println!("{}",t.get_value());
    println!("{}", x.get_value());
    //println!("{}",t.get_info());
    //foo(&t)
}


fn foo<T: Experiment<T> + std::fmt::Display>(item: &T) {
    println!("some value is {}", item.get_value())
}

#[derive(Copy, Clone, PartialEq, Debug)]
//struct Point1D<T>{
struct Point1D<T: Copy + std::ops::Add<T>> {
    x: T,
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Point2D<T> {
    x: T,
    y: T,
}

impl<T> std::ops::Add for Point1D<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Point1D<T>;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
        }
    }
}
impl<T> std::ops::Add for Point2D<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Point2D<T>;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> std::fmt::Display for Point1D<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.x)
    }
}
impl<T> std::fmt::Display for Point2D<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.x, self.y)
    }
}

impl<T> Experiment<T> for Point1D<T>
where
    T: Copy,
{
    fn get_info(&self) {
        println!("some info for 1D point");
    }
    fn get_value(&self) -> T {
        self.x
    }
}

impl<T> Experiment<T> for Point2D<T>
where
    T: Copy + std::ops::Add<Output = T>,
{
    fn get_info(&self) {
        println!("some info for 1D point");
    }
    fn get_value(&self) -> T {
        self.x + self.y // как сложить координаты?
    }
}

pub trait Experiment<T> {
    // как задать что бы также T реализовывал Copy?
    fn get_value(&self) -> T;
    fn get_info(&self) {
        println!("some info for 2D point");
    }
}
