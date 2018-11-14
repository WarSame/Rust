fn main() {
    println!("Final fib {}", fib(7));
}

fn fib(n: u32) -> u32 {
	if n == 1 || n == 0{
		return 1;
	}
	println!("n is {}", n);
	fib(n-1) + fib(n-2)
}
