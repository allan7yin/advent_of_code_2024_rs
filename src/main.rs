mod days;
fn main() {
    let day = days::ChristmasSaver{};
    println!("Day 1: ===");
    println!("{}", day.get_list_distance());
    println!("{}", day.get_similarity_score());

    println!("Day 2: ===");
    println!("{}", day.count_safe_reports());
    println!("{}", day.count_safe_reports_tol());

    println!("Day 3: ===");
    println!("{}", day.sum_of_mults());
    println!("{}", day.sum_of_mults_2());

    println!("Day 5: ===");
    println!("{}", day.get_med_valid_updates());
    println!("{}", day.get_med_postfix());

    println!("Day 6: ===");
    // println!("{}", day.count_visited_cells());
    // println!("{}", day.count_cycle_blocks());

    println!("Day 7: ===");
    println!("{}", day.find_poss_calibrations());
}
