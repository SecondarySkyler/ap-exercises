pub trait Printable {
    fn print(&self);
}

impl Printable for i32 {
    fn print(&self) {
       println!("{}", self); 
    }
}

impl Printable for String {
    fn print(&self) {
       println!("{}", self); 
    }
}

impl<T> Printable for Vec<T> where T: Printable {
    fn print(&self) {
        for item in self.iter() {
            item.print();
        }
    }
}

pub fn print<T: Printable>(g: T) {
    g.print();
}
