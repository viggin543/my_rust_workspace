use std::collections::HashMap; 
use itertools::Itertools;

fn main(){
    hashit();
}

pub fn hashit(){
    //Given a list of integers, use a vector and return the mean (the average value), 
    //median (when sorted, the value in the middle position),
    // and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    let da = vec![1,2,3,1,1,1];
    
    mean(&da);
    median(&da);
    mode(&da);
    mode_using_iter_tools(&da);


    
}
 

fn mode_using_iter_tools(da: &Vec<u8>){
    let mut max = 0;
    let mut max_key = 0;
    //why this needs the & ???????
    for (k,group) in &da.into_iter().group_by( |x| *x ) {
        let count = group.count();
        if count > max {
            max = count;
            max_key = *k; 
        }
    }
    println!("mode is ! {}", max_key);
}

fn mode(v : &Vec<u8>) {

    let mut m = HashMap::new();
    for x in v {
        let word_count = m.entry(x).or_insert(0);
        *word_count+=1;
    }
    if let Some((mode,_)) =  m.iter().max_by(|x,y| x.1.cmp(y.1)) {
        println!("mode using std max_by (the rust way) ! {}", mode);
    }
    
    let mut max_count = 0;
    let mut word = 0;
    for (k,v) in m {
       if v > max_count {
           max_count = v;
           word = *k;
       } 
    }
    println!("mode old school {:?}", word);
}

fn zzip(v : &Vec<u8>){
    let zero_vec = vec![1; v.len()];
    let word_count : HashMap<_,_> = v.iter()
        .zip(zero_vec.iter())
        .collect();
}

fn median(v : &Vec<u8>) -> u8 {
    let median = v[v.len()/2];
    println!("median -> {:?}", median);
    median
}

fn mean(v :&Vec<u8>) -> f32 {
    let sum = v.iter().fold(0.0,|a,b| a as f32 + *b as f32);
    println!("sum -> {:?}",sum);
    println!("mean -> {:?}",sum/v.len() as f32);
    sum
}