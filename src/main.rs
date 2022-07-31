const STARTING_BAGELS: i32 = 10;
const READY_TO_SERVE_BAGELS: i32 = 2;

fn main() {
    let mut bagel = STARTING_BAGELS;
    let ready = READY_TO_SERVE_BAGELS;
    let first_bagel: [f32; 3] = [100.0, 15.0, 5.0];
    let second_bagel: [f32; 3] = [101.0, 16.0, 6.0];
    let third_bagel: [f32; 3] = [102.0, 17.0, 7.0];
    let bagel_box: [[f32; 3]; 3] = [first_bagel, second_bagel, third_bagel];
    let bagel_calories: Vec<f32> = bagel_box
        .iter()
        .map(|bagel| calc_calorie_from_elements(bagel[0], bagel[1], bagel[2]))
        .collect();

    println!("ready to serve {} of {}", ready, bagel);
    println!("serving..!!");
    bagel -= ready;
    println!("now {} bagels left to serve", bagel);
    for bagel in bagel_calories {
        println!("a bagel with {:.2} calories", bagel)
    }
}

fn calc_calorie_from_elements(flour: f32, sugar: f32, butter: f32) -> f32 {
    (flour * 3.55) + (sugar * 3.86) + (butter * 7.16)
}
