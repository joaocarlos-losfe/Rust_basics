pub fn run()
{
    
    let mut _count = 0;

    //loop "infinito"
    loop 
    {
        _count += 1;

        println!("{}", _count);

        if _count == 10
        {
            break;
        }
    }

    _count = 0;

    //while loop

    while _count <=10
    {

        if _count % 2 == 0
        {
            println!("numero {} é par", _count)
        }
        else
        {
            println!("numero {} é impar", _count)
        }

        _count += 1;
    }

    //for range loop

    for x in 0..10
    {
        println!("loop range.. {}", x);
    }

}