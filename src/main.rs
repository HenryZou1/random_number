use rand::Rng;


fn main() {
	let mut number: u8 = 0;
	let mut num = magic_num::magic_number();
	//8th bit
	number = number | num;
	number = number << 1;
	num = magic_num::magic_number();
	//7
	number = number | num;
	number = number << 1;
	num = magic_num::magic_number();
	//6
	number = number | num;
	number = number << 1;
	num = magic_num::magic_number();
	//5
	number = number | num;
	number = number << 1;
	num = magic_num::magic_number();
	//4
	number = number | num;
	number = number << 1;
	num = magic_num::magic_number();
	//3
	number = number | num;
	number = number << 1;
	num = magic_num::magic_number();
	//2
	number = number | num;
	number = number << 1;
	num = magic_num::magic_number();
	//1
	number = number | num;
	println!("{}",number);
	number = rand::thread_rng().gen_range(0, 231) | num;
	println!("{}",number);
}
