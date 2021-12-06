// #[aoc_generator(day3)]
// pub fn bins(input: &str) -> Vec<&str> {
//     input.lines().collect()
// }

#[aoc(day3, part1)]
pub fn pt1<'a>(input: &str) -> String {
    
    let mut output = vec![0 as i16; input.lines().next().unwrap().len()];

    input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .for_each(
            |&x| 
            x.chars().enumerate().for_each(
                |x| output[x.0] += match x.1 {
                    '0' => -1,
                    '1' => 1,
                    _ => 0
                }
            )
        );

    let mut gamma_bin = vec![];
    let mut epsilon_bin = vec![];

    let mut gamma = 0;
    let mut epsilon = 0;

    output.iter().enumerate().for_each(
        |x| {
            gamma_bin.push(
                match *x.1 {
                    x if x < 0 => '0',
                    x if x > 0 => '1',
                    _ => '0'
                }
            );
            epsilon_bin.push(
                match *x.1 {
                    x if x < 0 => '1',
                    x if x > 0 => '0',
                    _ => '0'
                }
            );
        }
    );
    let gam_str: String = gamma_bin.into_iter().collect();
    let eps_str: String = epsilon_bin.into_iter().collect();

    gamma = i32::from_str_radix(&gam_str, 2).unwrap();
    epsilon = i32::from_str_radix(&eps_str, 2).unwrap();
    
    format!("{:?}, gamma={}, epsilon={}, answer={}", output, gamma, epsilon, gamma*epsilon)
}