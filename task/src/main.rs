fn main()
{
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();

    spawn(
        proc()
        {
            tx1.send("Hello from a task !".to_string());

            let message = rx2.recv();
            println!("{}", message);
        }
        );

    let message = rx1.recv();
    println!("{}", message);

    tx2.send("Goodbye from main !".to_string());
}
