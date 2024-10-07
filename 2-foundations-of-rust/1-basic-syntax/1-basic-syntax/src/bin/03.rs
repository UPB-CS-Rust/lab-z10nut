fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    // TODO
    let mut min = input[1];
    let mut max = input[1];
    for n in input { 
     if min > input[n] {
        min = input[n];
     }
     if max < input[n] {
        max = input[n];
     }
    }

    println!("{} is largest and {} is smallest", max, min);
}
