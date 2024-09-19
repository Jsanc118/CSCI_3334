fn is_even(n: i32) -> bool
{
if n%2 == 0
{
    return true;
}
    else
    {
    return false;
    }
}


fn main(){
let arr: [i32;10] = [1,2,3,4,5,6,7,8,9,15];

for idx in arr{

    if idx % 3 == 0 && idx % 5 == 0
    {
        println!("FizzBuzz")
    }
    else if idx % 5 == 0
    {
        println!("Buzz")

    }
    else if idx % 3 == 0 
    {
        println!("Fizz")
    }
    else if is_even(idx) == true
    {
        println!("The number is even")
    }
    else
    {
        println!("The number is odd")
    }
}
}