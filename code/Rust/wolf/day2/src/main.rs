use std::io::{self, BufRead};

type Level = i32;         // Each level in a report is an i32
type Delta = i32;         // The distance between two adjacent levels in a Report
type Report = Vec<Level>; // A report is a vector of levels


// Why create ToDeltas and to_deltas?  When I got to part 2, I saw that I would have to use the
// Deltas multiple times.  Storing them is one choice, but I went with recalculating them.  So I
// needed to hoist Delta calculation outside of the two number_of_safe... functions.  I could
// implement either:
//
//      to_deltas(&report)    or    report.to_deltas()
//
// It's a totally cosmetic choice.  I prefer the second choice.

trait ToDeltas {
    fn to_deltas(&self) -> Vec<Delta>;
}

impl ToDeltas for Report {
    fn to_deltas(&self) -> Vec<Delta> {

        // Calculate the Deltas, the difference between adjacent Levels in the Report.  It's the
        // Deltas that we look at to determine if the Report is Safe or Unsafe.  There are lots of
        // ways to calculate the Deltas.  I picked the conceptually simple (but maybe non-obvious)
        // way.  For each Delta what I care about is the Level to its left and the level to its
        // right. If I had those as a tuple, I could just do subtraction.  .zip of two lists gives
        // me a tuple.  So if I had a list of left-Levels and a list of right-Levels I could do it.
        // The left-Levels is just all of the Levels except the last one.  The right-Levels is all
        // of the Levels starting from the second one.  If I pair these up with .zip each tuple
        // will have two adjacent Levels in it, and I will get exactly as many tuples as there are
        // gaps (and each gap is a Delta) between the Levels.

        self[..self.len() - 1]
            .iter()
            .zip(&self[1..])
            .map(|(a, b)| b - a)
            .collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ReportSafety {
    Safe,
    Unsafe,
}

fn is_safe(safety: &ReportSafety) -> bool {
    // TODO: I don't understand why I need the & when ReportSafety is Copy
    *safety == ReportSafety::Safe
}

fn read_in_the_reports() -> Vec<Report> {
    // No parameters.  We get all our data from stdin.
    io::stdin()
        .lock()
        .lines()
        .map(|line| {
            // Instead of first reading all the lines, and then separately parsing each line as I
            // did in day1, I'm going to parse each line into a Report as I'm reading.

            // Here, line is a Result<String, std::io::Error>.  .expect unwraps it and I get the
            // string itself, or else we panic.
            let report_str = line.expect("Failed to read line");

            // With the string, .split_whitespace gives me an iterator.  The input is well-formed,
            // so each step in the iterator is a string of digits.  The .map returns the actual
            // i32 or as we're calling it: a Level.  The .collect gathers them into a Vec<Level>,
            // that is, a Report.
            report_str
                .split_whitespace()
                .map(|num_str| num_str.parse::<Level>().unwrap())
                .collect()
        })
        // This final .collect gathers the individual Reports into a Vec<Report>, and that is the
        // result of this function.
        .collect()
}

fn check_report(report: &Report) -> ReportSafety {
    let deltas = report.to_deltas();

    // Check Condition 1: The magnitude of all the Deltas must be in range 1..=3
    if !deltas.iter().all(|&delta| (1..=3).contains(&delta.abs())) {
        ReportSafety::Unsafe
    } else {
        // Check Condition 2: All Deltas must have the same sign
        let first_sign = deltas[0].signum();
        if deltas.iter().all(|&delta| delta.signum() == first_sign) {
            ReportSafety::Safe
        } else {
            ReportSafety::Unsafe
        }
    }
}

fn check_unsafe_report_with_problem_dampener_using_brute_force(unsafe_report: &Report) -> ReportSafety {
    // Brute-force.  This just a first pass.  I'll look at the debug out to see the corrected
    // Reports and maybe that will help me figure out a smarter way to do it.

    if cfg!(debug_assertions) {
        // I probably don't need this whole clause.
        if is_safe(&check_report(unsafe_report)) {
            println!("Called check_unsafe_report_with_problem_dampener_using_brute_force with a Report that was actually Safe.");

            // Rust complained when I tried to use a tail expression here.  I think it's because of
            // the cfg!.
            return ReportSafety::Safe;
        }
    }

    // Nothing smart, we're allowed to fix the Report by removing one Level, so just try it for
    // each Level in turn.  Break out early when we find a Safe result.
    for i in 0..unsafe_report.len() {
        let mut test_report = unsafe_report.clone();
        // Also nothing smart.  We could have stuffed together two slices (before and after the
        // removed Level), but we'll just us Vec::remove.
        test_report.remove(i);
        if is_safe(&check_report(&test_report)) {
            if cfg!(debug_assertions) {
                // Because Report is a type alias, I can't just print or log it.  Ugh.
                let levels = unsafe_report
                    .iter()
                    .map(|n| format!("{}", n))
                    .collect::<Vec<String>>()
                    .join(", ");
                println!("The Unsafe Report [{}] became Safe when we removed the Level at position {}.", levels, i);
            }

            // A tail expression doesn't exit the function, just the loop
            return ReportSafety::Safe;
        }
    }
    ReportSafety::Unsafe
}

fn number_of_safe_reports(reports: &[Report]) -> i32 {
    reports
        .iter()
        .map(check_report)
        .filter(is_safe)
        .count() as i32
}

fn number_of_safe_reports_using_problem_dampener(reports: &[Report]) -> i32 {
    reports
        .iter()
        .map(|report| {
            if is_safe(&check_report(report)) || is_safe(&check_unsafe_report_with_problem_dampener_using_brute_force(report)) {
                ReportSafety::Safe
            } else {
                ReportSafety::Unsafe
            }
        })
        .filter(is_safe)
        .count() as i32
}

fn main() {
    let reports = read_in_the_reports();

    println!(
        "Day 2, part 1: there are {} safe reports.",
        number_of_safe_reports(&reports)
    );

    println!(
        "Day 2, part 2: there are {} safe reports when using the problem dampener.",
        number_of_safe_reports_using_problem_dampener(&reports)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    // This is the same list of reports from the problem description.  I'll compare against the
    // answers given in the same place.
    fn test_reports() -> Vec<Report> {
        vec![
            vec![7, 6, 4, 2, 1],  // Unsafe
            vec![1, 2, 7, 8, 9],  // Unsafe
            vec![9, 7, 6, 2, 1],  // Safe
            vec![1, 3, 2, 4, 5],  // Unsafe
            vec![8, 6, 4, 4, 1],  // Unsafe
            vec![1, 3, 6, 7, 9],  // Safe
        ]
    }

    #[test]
    fn test_day2_part1() {
        let reports = test_reports();
        assert_eq!(number_of_safe_reports(&reports), 2);
    }

    #[test]
    fn test_day2_part2() {
        let reports = test_reports();
        assert_eq!(number_of_safe_reports_using_problem_dampener(&reports), 4);
    }
}
