mod days;
fn main() {
    let day = days::ChristmasSaver{};
    println!("Day 1: ===");
    println!("{}", day.get_list_distance());
    println!("{}", day.get_similarity_score());

    println!("Day 2: ===");
    println!("{}", day.count_safe_reports());
}
