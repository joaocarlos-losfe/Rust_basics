enum Movimentos
{
    Cima,
    Baixo,
    Esquerda,
    Direita
}

fn mover_personarem(mov: Movimentos)
{
    //similar a um switch
    match mov
    {
        Movimentos::Cima => println!("moveu pra cima"),
        Movimentos::Baixo => println!("moveu pra baixo"),
        Movimentos::Esquerda => println!("moveu pra esquerda"),
        Movimentos::Direita => println!("moveu pra direita")
    }
}

pub fn run()
{
    let mut _w = Movimentos::Cima; 
    let mut _s = Movimentos::Baixo; 
    let mut _a = Movimentos::Esquerda; 
    let mut _d = Movimentos::Direita;
    
    mover_personarem(_w);
    mover_personarem(_s);
    mover_personarem(_a);
    mover_personarem(_d);
}