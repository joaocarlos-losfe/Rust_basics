use std::env;

pub fn run()
{
    let args: Vec<String> = env::args().collect();

    //  0                             1     2    3
    //["target/debug/rust_exemples", "10", "+", "10"]

    println!("argumentos repassados: {:?}", (args));

    let mut _n1 = args[1].to_string();
    let op = args[2].to_string();
    let mut _n2 = args[3].to_string();

    //converter string em inteiros
    let n1 = _n1.parse::<i32>().unwrap();
    let n2 = _n2.parse::<i32>().unwrap();

    println!("{}", op);

    if args.len() != 4
    {
        println!("numero de argumentos invalido");
        println!("sequencia correta: valor, operação, valor")
    }
    else if op == "+"
    {
        println!("soma: {}", n1 + n2);
    }
    else if op == "-"
    {
        println!("subtracão {}", n1 - n2);
    }
    else
    {
        println!("operador invalido");
    }
    
}