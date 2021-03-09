pub fn run()
{
    //basico. parecido com outras linguagens

    let age: u8 = 17;

    if age >= 21{
        println!("mair que 18");
    }
    else if age == 18
    {
        println!("pode beber");
    }
    else
    {
        println!("menor ok");
    }

    // forma abreviada
    //operador ternario
    let is_older = if age >= 21 {true} else {false} ;

    println!("{}", is_older);

    let mut _is_maior : bool = 18 < 17; // false

    println!("{}", _is_maior);

}