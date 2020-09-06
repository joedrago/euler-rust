mod problems;

fn main() {
    let all = problems::all();
    for problem in all {
        println!("\n--------------------------------------------------");
        println!("{}", problem.title());

        println!("\n{}\n", problem.description());

        problem.test();
        problem.answer();
    }
}
