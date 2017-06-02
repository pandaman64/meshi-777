use std::io::stdin;
use std::io::BufRead;
use std::option::Option;
use std::collections::hash_map::HashMap;
use std::rc::Rc;

#[derive(Debug,Clone)]
enum ConsCell{
    Nil,
    Cons(usize,Rc<ConsCell>)
}

#[derive(Debug)]
struct ConsCellIter<'a>{
    cell: &'a ConsCell
}

impl ConsCell{
    fn new() -> ConsCell{
        ConsCell::Nil
    }

    fn singleton(v: usize) -> ConsCell{
        ConsCell::new().cons(v)
    }

    fn cons(&self, v: usize) -> ConsCell{
        ConsCell::Cons(v, Rc::new(self.clone()))
    }

    fn iter(&self) -> ConsCellIter{
        ConsCellIter{ cell: self }
    }
}

impl<'a> Iterator for ConsCellIter<'a>{
    type Item = &'a usize;

    fn next(&mut self) -> Option<Self::Item>{
        use ConsCell::*;
        match self.cell{
            &Nil => None,
            &Cons(ref v,ref c) => {
                self.cell = c;
                Some(v)
            }
        }
    }
}

const N: usize = 777;

type Path = ConsCell;
type Memo = Vec<Option<Vec<Path>>>;

fn solve<I: Iterator<Item=usize> + Clone>(memo: &mut Memo,n: usize,prices: I){
    if n == 0 || memo[n].is_some(){
        return;
    }
    let mut answers = vec![];
    for price in prices.clone(){
        if n == price{
            answers.push(ConsCell::singleton(price));
        }
        else if n > price{
            solve(memo,n - price,prices.clone());
            if let Some(ref v) = memo[n - price]{
                for p in v.iter().cloned(){
                    answers.push(p.cons(price));
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

    /*for m in memo.iter(){
        println!("{:?}",m);
    }*/

    for recipt in memo[N].as_ref().unwrap().iter(){
        let s = recipt.iter().map(ToString::to_string).collect::<Vec<_>>().join(" + ");
        println!("{}",s);
    }
}
