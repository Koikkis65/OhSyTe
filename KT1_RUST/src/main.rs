// Aloitusluvut 0 ja 1 kuten näkyy
fn main() {
    let mut first_num: u128 = 0;
    let mut second_num: u128 = 1;
    let mut count: u128 = 0;

    while count < 1000 {
        println!("Num count: {}, fib: {}", count,first_num);
        let temp: u128 = first_num;
        first_num = first_num + second_num;
        second_num = temp;
        count += 1;
    }
}