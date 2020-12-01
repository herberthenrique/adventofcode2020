use std::fs;


mod report_repair;


fn get_inputs () -> Vec<i32> {
    let inputs: Vec<i32> = (match fs::read_to_string("./src/data/day_one") {
        Ok(s) => s,
        Err(e) => panic!("Could not read file: {}", e)
    })
        .split("\n")
        .collect::<Vec<&str>>()
        .iter().map(|&s| match s.parse::<i32>() {
            Ok(i) => i,
            Err(_) => 0 // ignore a bad parse, was probably a new line
        })
        .filter(|&i| i != 0)
        .collect();

        return inputs;
}


fn main() {
    let input = get_inputs();
    let result: i32 = report_repair::find_error_part_one(input.clone());
    let result_two: i32 = report_repair::find_error_part_two(input.clone());

    println!(" result_one: {}", result);
    println!(" result_two: {}", result_two);
}