// 变量和可变性
fn main() {
	let mut x = 3;
	println!("x is {}.", x);
	x = 4;
	println!("x is {}.", x);

	// 常量
	const CITY: &str = "ChengDu";
	println!("{}", CITY);

	// 变量的遮蔽
	let y = 3;
	let y = y + 1;
	println!("y is {}", y); // 4
	{
		let y = 3 * y;
		println!("y is {}", y); //12
	}
	println!("y is {}", y); // 4
}