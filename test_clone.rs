use prism3_function::BoxBiFunctionOnce;

fn main() {
    let func = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x + *y);
    let cloned = func.clone(); // This should fail if not Clone
    println!("Clone successful");
}
