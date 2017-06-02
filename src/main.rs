use std::io::stdin;
use std::io::BufRead;
use std::option::Option;
use std::collections::hash_map::HashMap;

const N: usize = 777;

type Memo = Vec<Option<Vec<usize>>>;

fn solve<I: Iterator<Item=usize> + Clone>(memo: &mut Memo,n: usize,prices: I){
    if n == 0 || memo[n].is_some(){
        return;
    }
    let mut answers = vec![];
    for price in prices.clone(){
        if n == price{
            answers.push(price);
        }
        else if n > price{
            solve(memo,n - price,prices.clone());
            if let Some(ref v) = memo[n - price]{
                if !v.is_empty(){
                    answers.push(price);
                }
            }
        }
    }
    memo[n] = Some(answers);
}

fn main() {
    let mut memo: Memo = vec![None; N + 1];

    let mut prices = HashMap::new();

    let stdin = stdin();
    let mut stdin = stdin.lock();

    // skip number of lines
    stdin.read_line(&mut String::new()).unwrap();
    let mut lines = stdin.lines();
    while let Some(Ok(line)) = lines.next(){
        let mut splitted = line.split(' ');
        let price = splitted.next().unwrap().parse::<usize>().unwrap();
        let name = splitted.next().unwrap();

        prices.entry(price).or_insert(vec![]).push(name.to_owned());
    }

    solve(&mut memo,N,prices.keys().cloned());

    for m in memo.iter(){
        println!("{:?}",m);
    }
}
