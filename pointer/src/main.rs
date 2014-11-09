fn main()
{
    let mut x = 5i;

    let p = proc()
    {
        x * x
    };

    println!("{}", p());

    x = 6i;
}
