use std::io::{self, BufRead, Error, ErrorKind};


// Standard input should contain a line for each generation, a space, a score.
// The input must be sorted by generation.
//
// You might create this input with something like:
// grep 'E: ' space_invaders_*.log | cut -d ' ' -f 6-7 | sort -n > generations-sorted.log
// ... for the Atari CGP experiments.
fn main() {
    const N : usize = 5; 
    let stdin = io::stdin();

    // Produces an iterator of Result<String>
    // Calling next gives an Option<Result<String>>
    let mut lines = stdin.lock().lines();

    // We're only going to output something when the mean changes:
    let mut last_gensum = GenSummary { generation: 0, mean: 0.0, stderr: 0.0 };

    loop {

        let mut vec = Vec::new();
        for _l in 0..N {
           if let Some(result) = lines.next() {
              vec.push(result.and_then(parse).unwrap()); // Will panic on EOF
           }
        }

        assert!(same_generation(&vec), "Different generations found: {:?}", vec);
        let gensum = summary(&vec);
        if gensum.mean != last_gensum.mean {
            println!("{} {} {}", gensum.generation, gensum.mean, gensum.stderr);
            last_gensum = gensum;
        }
    }

}

#[derive(Debug)]
struct LogLine {
    generation: i64,
    score: f64,
}

#[derive(Debug)]
struct GenSummary {
    generation: i64,
    mean: f64,
    stderr: f64,
}

fn summary(vec: &Vec<LogLine>) -> GenSummary {
    let m = mean(&vec);
    GenSummary {
        generation: vec.first().unwrap().generation,
        mean: m,
        stderr: sem(m, &vec),
    }
}

fn mean(numbers: &Vec<LogLine>) -> f64 {
    numbers.iter().map(|l| l.score).sum::<f64>() / numbers.len() as f64
}

fn sem(mean: f64, numbers: &Vec<LogLine>) -> f64 {
    let n = numbers.len() as f64;
    let variance = numbers.iter().map(|value| {
        let diff =  mean - value.score;
        diff * diff
    }).sum::<f64>() / (n-1.0);

    let std = variance.sqrt();

    std / n.sqrt()
}

fn same_generation(vec: &Vec<LogLine>) -> bool {
    let g = vec.first().unwrap().generation;
    vec.iter().all(|i| i.generation == g)
}

fn parse(line: String) -> Result<LogLine, Error> {
    let mut words = line.split_whitespace();
    let maybe_gen = words.next().and_then(|s| s.parse::<i64>().ok());
    let maybe_score = words.next().and_then(|s| s.parse::<f64>().ok());
  
    match (maybe_gen, maybe_score) {
        (Some(generation), Some(score)) => Ok(LogLine { generation, score }),
        _ => Err(Error::new(ErrorKind::Other, "Cannot parse line"))
    }
        
}
