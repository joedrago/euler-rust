use super::Problem;

pub struct Problem001 {}

impl Problem001 {
    fn sum_of_multiples_of_3_and_5(max: u32) -> u32 {
        let mut sum = 0;
        for n in 1..max {
            if (n % 3 == 0) || (n % 5 == 0) {
                sum += n;
            }
        }
        return sum;
    }
}

impl Problem for Problem001 {
    fn title(&self) -> &'static str {
        "Problem 1: Multiples of 3 and 5"
    }
    fn description(&self) -> &'static str {
        "If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. \
         The sum of these multiples is 23.\n\nFind the sum of all the multiples of 3 or 5 below 1000."
    }

    fn test(&self) {
        println!(
            "[Test] Sum of natural numbers < 10: {}",
            Problem001::sum_of_multiples_of_3_and_5(10)
        );
    }

    fn answer(&self) {
        println!(
            "[Answer] Sum of natural numbers < 1000: {}",
            Problem001::sum_of_multiples_of_3_and_5(1000)
        );
    }
}
