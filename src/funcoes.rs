pub fn run()
{
    imprime("joao", 23);

    //Closure ou funções internas

    let add_num_clossure = |n1:i32, n2: i32| n1 + n2;
    println!("add_num_clossure {}", add_num_clossure(10,10));

    let mut _n3:i32 = 10;
    let add_num_clossure_external_var = |n1:i32, n2: i32| n1 + n2 + _n3;
    println!("add_num_clossure_external_var {}", add_num_clossure_external_var(10,10));
}

//função não publica com passagem de paramentros sem retorno
fn imprime(nome:&str, idade:u8)
{
    println!("{} {}", nome, idade);
}

//função publica com passagem de parametro com retorno
pub fn soma(n1:i32, n2:i32) -> i32
{
    
    n1 + n2
}
