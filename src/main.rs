const STARTING_BAGELS: i32 = 10;
const READY_TO_SERVE_BAGELS: i32 = 2;

fn main() {
    let (mut bagel, ready) = (STARTING_BAGELS, READY_TO_SERVE_BAGELS);

    println!("ready to serve {} of {} bagels", ready, bagel);
    println!("serving..!!");
    bagel -= ready;
    println!("now {} bagles left to serve", bagel);
    println!(
        "a bagel with {:.2} calories",
        calc_calorie_from_elements(100.0, 100.0, 100.0)
    );
}

fn calc_calorie_from_elements(flour: f32, sugar: f32, butter: f32) -> f32 {
    flour * 3.55 + sugar * 3.86 + butter * 7.16
}
