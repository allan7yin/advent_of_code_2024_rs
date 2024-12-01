mod days;
fn main() {
    let day1 = days::ChristmasSaver{};
    println!("{}", day1.get_list_distance());
    println!("{}", day1.get_similarity_score());
}
