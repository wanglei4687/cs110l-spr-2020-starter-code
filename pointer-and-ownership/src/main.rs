fn main() {
    // immutable and mutable references (or borrows)
    let mut n: i32 = 1;
    {
	let nptr: &i32 = &n;
	// n = 2; <- invalid!
    }
    n = 2; // ok!
    {
	let nptr: &mut i32 = &mut n;
	*nptr = 3;
	// n = 2; <- still invalid!
    }

    // ownership
    let mut s: String = String::from("Hello");
    s.push_str(" world!"); // Owner can mutable its value
    let s2 = s; // pass of ownership
    // println!("{}", s}; <- invalid! s has moved
    println!("{}", s2);

    let s4 = {
	let s3 = s2 + " Nice to see you again!";
	s3 // pass ownership by returning from a scope
    };

    // Note that s, s2, s3 are all moved or inaccessible at this point
    // only s4 is live

    let s5 = s4.clone(); // deep copy
    println!("{}, {}", s4, s5); // no problem

    // more complex examples of borrow checking with data structures

    let mut v: Vec<i32> = vec![1, 2, 3];
    let n: &i32 = &v[1]; // a pointer to an element of a vec is like an iterator
    // v.push(5); <- invalid! cannot mutate v while any element is borrowed

    // independent borrows on struct fields
    struct Point { x: f32, y: f32 }
    let mut p = Point { x: 0.0, y: 0.0 };
    {
	let n: &f32 = &p.x;
	// this is actually ok! can borrow x and y separately, as changing one
	// will never affect the other
	let m: &mut f32 = &mut p.y;
	*m = 1.1;
    }

    // different kinds of borrows in implementaion.
    impl Point {
	// &self means p.print() imutably borrows p.
	fn print(& self) {
	    println!{"({}, {})", self.x, self.y};
	}

	// &mut self means p.incr_x() mutably  borrows p
	fn incr_x(&mut self) {
	    self.x += 1.0;
	}

	// self means p.consume() moves p
	fn consume(self) {}
    }

    p.print();
    p.incr_x();
    p.consume();
    // p.print(); <- invalid! p has moved

    // borrowing inner fields of enums.
    enum List<T> { Nil, Cons(T, Box<List<T>>) }
    let mut l: List<i32> = List::Cons(3, Box::new(List::Nil));
    match &1 {
	List::Cons(n, _) => {
	    // *n = 3; <- invalid! cannot assign to immutable borrowed content
	    println!("{}", *n);
	}
	List::Nil => {}
    }
}
