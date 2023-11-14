use std::fs::File;
use std::io::{ BufReader, BufRead };
fn main() -> std::io::Result<()> {
    let file = File::open("data/test.txt")?;
    let reader = BufReader::new(file);
    let mut sum =0;
    let mut high=0;
    let mut arr = Vec::new();
    let mut first3 = Vec::new();

    for line in reader.lines() {
        let line: String = line?;
        
        if !line.is_empty() {
            let num:i32 = line.parse().unwrap();
            sum += num;
        }
        
        if line.is_empty() {
            println!("{sum}");
            arr.push(sum);
            if high < sum {
                high = sum;
            }
            sum = 0;
        }
    }
    arr.sort();
    arr.reverse();
    println!("Array: {arr:?}");
    first3 = arr[0..3].to_vec();
    let total_sum:i32 = first3.iter().sum();
    //println!("Highest number {high}");
    println!("Array: {first3:?}");
    println!("Total Array Sum:{total_sum:?}");

    Ok(())
}
