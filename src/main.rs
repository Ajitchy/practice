
/*fn main() {
    let is_learning =true;
    let explict_type: bool =true;
    println!("is_learning: {}", is_learning);
}*/
/*
fn main() {
    /* Hello world project
    println!("Hello, world!");
     */
    /* Signed integer
    let number_small:i8=-120;
    println!("i128 min :{}",i128:: min_value());
    println!("i128 max :{}",i128:: max_value());
     */
    /*
    //Integer Overflow
    let number_small: u8 = 256;
    println! ("u8:{}", number_small);
     */
    /*
    //Error at run-time "Panic"
    let number_small:u8=255;
    println!("u8:{}", number_small);
    //case int overflow
    let number_small= number_small + 1;
    println!("u8:{}",number_small);
    */
    /*
    let number_small:u8=255;
    println!("u8:{}", number_small);
    //cause int overflow
    let result =number_small.overflowing_add(1);
    println!("{:?}",result);

     */
    /*
    //Arithmetic Operators
    //Addition
    println!("1+5={}",1+5);
    //subtraction
    println!("5-1={}",5-1);
    //Multiplication
    println!("1*5={}",1*5);
    //division
    println!("5/1={}",5/1);
     */
    /*
    //Relational operator
    println!("5>3={}",5>3);
    println!("5<=3={}",5<=3);
    println!("5>=3={}",5>=3);
    println!("5==3={}",5==3);
    println!("5!=3={}",5!=3);
     */

    /*
    //Assignment Operator

    let mut a=5;
    println!("a:{}",a);
    a+=3;
    println!("a+=3(a=a+3):{}",a);
    a-=3;
    println!("a-=3(a=a-3):{}",a);
    a*=3;
    println!("a*=3(a=a*3):{}",a);
    a/=3;
    println!("a/=3(a=a/3):{}",a);
    a%=3;
    println!("a%=3(a=a%3):{}",a);
     */
    /*
    //Logical Operator
    println!("5>=3&&1<=2:{}",5>=3&&1<=2);
    println!("5==3||1<=2:{}",5==3||1<=2);
    println!("!true:{}",!true);
     */
    /*
    //Function
    println!("Hello, world!");
    another_function();
}
fn another_function(){
    println!("Another function");
     */
    /*
    //Function parameters
    another_function(5);
}
fn another_function(x:i32){
    println!("The value of x is:{}",x);

     */
 */
    /*
    //Function parameters
    print_labeled_measurment(5,'h');
}
fn print_labeled_measurment(value:i32,unit_label:char){
    println!("The measurment is:{}{}",value,unit_label);
     */

    /*
    //Function body contain statements and expression
    let y={
        let x=3;
        x+1 //This line is expression so it doesn't have semicolon
    };
    println!("The value of y is:{}",y);
}
    */

/*
//Function with return value
    fn five ()-> i32{
    5
}
    fn main(){
    let x=five();
    println!("The value of x is:{}",x);
    }

 */
/*
//Another example of function with return value
fn main(){
    let x=plus_one(5);
    println!("The value of x is:{}",x);
}
fn plus_one(x:i32)->i32{
    x+1 // This is expression so no semicolon putting semicolon will give error
}
 */
/*
//Repeating code with loop
fn main(){
    loop {
        println!("again!");
    }

}
 */
/*
fn main(){
    let mut count =0;
    'counting_up: loop{
        println!("count={}", count);
        let mut remaining=10;
        loop{
            println!("remaining={}", remaining);
            if remaining ==9{
                break;
            }
            if count ==2{
                break 'counting_up;
            }
            remaining -=1;
        }
        count +=1;
    }
    println!("End count ={}", count);
}
 */
/*
//Reterning values from loops
fn main(){
    let mut counter=0;
    let result = loop{
        counter +=1;
        if counter ==10{
            break counter*2;
        }
    };
    println!("This result is{}",result);
}
*/
/*
//Using while instead of loop,if,else,break
fn main(){
    let mut number=3;
    while number!=0{
        println!("{}", number);
        number -=1;
    }
    println!("LIFTOFF!!!");
}
 */
/*
//Looping through a collecting using for
fn main(){
    let a=[10,20,30,40,50];
    let mut index=0;
    while index<5{
        println!("the value is: {}", a[index]);
        index +=1;
    }
}

 */
/*
//Above code is error prone; we could cause it to panic if the index value or
//test condition are incorrect. So we use for loop
fn main(){
    let a=[10,20,30,40];
    for element in a{
        println!("the value is:{}",element);
    }
}

 */
/*
//Counter code that was written in while can be written with more safety as
fn main(){
    for number in (1..4).rev(){
        println!("{}",number);
    }
    println!("LIFTOFF!!!");
}
 */
/*
//Fahrenhiet to celcius converter
fn main(){
    let f:f32=38.0;
    let  celsius=(((f) as i32 -32)*5)/9;
    println!("Celsius value is:{}",celsius);
}
 */
/*
//Fibonnaci series
fn fibonnaci(n:u32)-> u32{
    match n{
        0=>0,
        1=>1,
        _=>fibonnaci(n-1)+ fibonnaci(n-2)
    }
}
fn main(){
   println!("Fibonnaci generator");
    println!("{}",fibonnaci(0));
    println!("{}",fibonnaci(5));
    println!("{}",fibonnaci(9));
}
 */
/*
//Fibonnaci using for
fn fibonnaci(n:u32)-> u32{
   let mut sum=0;
    let mut last=0;
    let mut curr=1;
    for _i in 1..n{
        sum=last +curr;
        last=curr;
        curr=sum;
    }
    sum
}
fn main(){
    println!("{}",fibonnaci(2));
}
 */
/*
//Ownership
//Below code will give error because as s1 value is dropped and assigned to s2
fn main(){
    let s1=String::from("hello");
    let s2=s1;
    println!("{}",s1);
}
 */
/*
//ownership and function
fn main(){
    let s=String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                                  // ... and so is no longer valid here

    let x=5;  // x comes into scope

    makes_copy(x);   // x would move into the function,
                               // but i32 is Copy, so it's okay to still
                               // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
// special happens.

fn takes_ownership(some_string:String){ // some_string comes into scope
    println!("{}",some_string);
}// Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.
fn makes_copy(some_integer:i32){  // some_integer comes into scope
    println!("{}",some_integer);
}  // Here, some_integer goes out of scope. Nothing special happens.

 */
/*
//Return Value and Scope
fn main(){
    let s1=gives_ownership();
    let s2=String::from("hello");
    let s3=takes_and_gives_back(s2);

}
fn gives_ownership()->String{
    let some_string=String::from("yours");
    some_string
}
fn takes_and_gives_back(a_string:String)->String{
    a_string
}

 */
/*
//It is possible to return multiple value using tuple
fn main(){
    let s1=String::from("hello");
    let (s2,len)= calculate_length(s1);
    println!("The length of '{}'is {}", s2,len);
}
fn calculate_length(s:String)->(String,usize){
    let length=s.len();
    (s,length)
}

 */
/*
//References
//The issue with the tuple is we have to return the string to the calling function
//so we can still use the string after the call to calculate_length, because the
//string was moved into calculate_length
fn main(){
    let s1=String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}",s1,len);
}
fn calculate_length(s:&String)->usize{
    s.len()
}
 */
