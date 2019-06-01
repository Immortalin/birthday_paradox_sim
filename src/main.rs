extern crate plotlib;
extern crate rayon;

use getopts::Options;
// use plotlib::line::Line;

use plotlib::page::Page;
use plotlib::style::Line;
use plotlib::view::ContinuousView;

use rand::{thread_rng, Rng};
use rayon::prelude::*;

use std::collections::HashSet;
use std::env;

fn main() {

    // ==== SETUP CODE ====
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("n", "", "set number of cycles to run", "CYCLES");
    opts.optopt(
        "p",
        "",
        "set number of people to test for birthday collisions",
        "PEOPLE",
    );
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let mut cycles = 100;
    let mut number_of_people = 10;
    if let Some(n) = matches.opt_str("n") {
        cycles = n.parse::<i64>().unwrap();
    }

    if let Some(p) = matches.opt_str("p") {
        number_of_people = p.parse::<i64>().unwrap();
    }
    let mut probability_list: Vec<(f64, f64)> = Vec::new();

    // ==== END SETUP BOILERPLATE ====
    for i in 0..number_of_people {
        probability_list.push((i as f64 + 1., run_simulation(i, cycles)));
    }

    let l = plotlib::line::Line::new(&probability_list)
        .style(plotlib::line::Style::new().colour("burlywood"));
    // Line.style::new().colour("#35c788")

    let v = ContinuousView::new()
        .add(&l)
        // .add(&s2)
        .x_range(1., number_of_people as f64)
        .y_range(0., 1.)
        .x_label("Number of people in room")
        .y_label("Probability of having the same birthday");

    // A page with a single view is then saved to an SVG file
    let graph_name = "graph.svg";
    match Page::single(&v).save(graph_name) {
        Ok(_) => println!("Graph saved to graph_name"),
        Err(_) => eprintln!("Graph creation failure!"),
    }
}

fn run_simulation(number_of_people: i64, cycles: i64) -> f64 {
    println!("Simulating Birthday Paradox with {} cycles", cycles);

    // for _ in 0..cycles {
    let probabilities_sum = (0..cycles)
        .into_par_iter()
        .map(|_| {
            let same_birthday = simulate_paradox(number_of_people);
            if same_birthday {
                println!("Same birthday found!");
                1
            } else {
                println!("No same birthday :(!");
                0
            }
        })
        .reduce(|| 0, |x, y| x + y);
    let probability = probabilities_sum as f64 / cycles as f64;

    println!(
        "Probability of having the same birthday in a room of {n} people with {i} cycles of simulation is {p}",
        n = number_of_people,
        i = cycles,
        p = probability
    );
    probability
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}


// Returns a True if a number of people has the same birthday, False otherwise
fn simulate_paradox(number_of_people: i64) -> bool {
    // Generates uniform distribution
    let mut rng = thread_rng();

    // #[derive(PartialEq, Eq, Hash)]
    let mut birthdays: Vec<i64> = Vec::new();
    for _ in 0..number_of_people {
        let day = rng.gen_range(0, 365);
        println!("{}", day);
        birthdays.push(day);
    }
    // Having same birthday is the opposite of having unique birthdays
    !has_unique_birthdays(&birthdays)
}

fn has_unique_birthdays(birthdays: &[i64]) -> bool {
    let mut uniq = HashSet::new();
    // Tests for uniqueness, iter.all() returns false if a single element returns false
    birthdays.iter().all(|day| uniq.insert(day))
}
