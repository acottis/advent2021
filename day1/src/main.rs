fn main() {
    // P1
    let start = rdtsc();
    let p1 = part1();
    println!("{}", rdtsc() - start);

    // P2
    let start = rdtsc();
    let p2 = part2();
    println!("{}", rdtsc() - start);
    
    // Results
    println!("Part 1: {p1}\nPart 2: {p2}");
}

#[inline(always)]
fn part2() -> u32 {
    let nums = parse_inputs();
    let len = nums.len();

    // Count the number of times it is higher
    let mut count = 0;
    for i in 0..len-3 {
        let group_a = nums[i] + nums[i+1] + nums[i+2];
        let group_b = nums[i+1] + nums[i+2] + nums[i+3];
        if group_a < group_b {
            count += 1;
        }
    }
    count
}

#[inline(always)]
fn part1() -> u32 {
    let nums = parse_inputs();
    let len = nums.len();

    // Count the number of times it is higher
    let mut count = 0;
    for i in 0..len-1 {
        if nums[i] < nums[i+1]{
            count += 1;
        }
    }
    count
}

fn parse_inputs() -> Vec<u16>{
    // Read file at compile time
    const INPUT: &str = include_str!("../input.txt");
    
    // Split by new line
    let lines = INPUT.lines();
    
    // Collect the u16 version of the numbers
    lines
        .map(line_to_u16)
        .collect::<Vec<u16>>()
}

#[inline(always)]
fn line_to_u16(line: &str) -> u16{
    line.parse::<u16>().unwrap()
}

#[inline(always)]
fn rdtsc() -> u64 {
    unsafe { core::arch::x86_64::_rdtsc() }
}