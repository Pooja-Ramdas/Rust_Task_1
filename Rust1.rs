struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 
    {
        let mut output: Vec<i32> = vec![];
        
        for x in nums{
            if output.iter().position(|num: &i32| num==x)== None{
                output.push(*x);
                
            }
        }
        let len: usize = output.len();
        
        len as i32
    }
}

fn main(){
    println!("{}",Solution::remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]));
}