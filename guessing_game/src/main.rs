use std::io;
use std::rand;

fn  main() {
    println!("Guess the number !");

    let secret_number = (rand::random::<uint>() % 100u) + 1u;

    loop
    {
        println!("Please input your guess.");

        let input = io::stdin().read_line()
            .ok()
            .expect("Failed to read line");

        let input_num: Option<uint> = from_str(input.as_slice().trim());

        let num = match input_num
        {
            Some(num)   =>  num,
            None        =>  
            {
                println!("Please input a number !");
                continue ;
            }
        };

        println!("You guessed : {}", num);

        match cmp(num, secret_number)
        {
            Less    =>  println!("Too small !"),
            Greater =>  println!("Too big !"),
            Equal   =>  
            {
                println!("You win !");
                return ;
            },
        }
    }
}

fn  cmp(src: uint, trg: uint) -> Ordering
{
    if src < trg
    {
        Less
    }
    else if src > trg
    {
        Greater
    }
    else
    {
        Equal
    }
}
