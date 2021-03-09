pub fn run()
{
    //tipos definidos em sequencia
    let mut _person: (&str, &str, i8) = 
    (
        "Joao",
        "sousa",
        37,
    );

    println!("{} {} {}", _person.0, _person.1, _person.2);

}