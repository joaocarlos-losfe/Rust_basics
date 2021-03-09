pub fn run()
{
    //tipo string explicita mutavel inicialiada 
    let mut _hello = String::from("hello ");

    //tipo string explicita mutavel não inicialiada 
    let mut _strs = String::default();

    println!("{}", _hello);

    //obtendo o numero de caracteres da string
    println!("{}", _hello.len());

    //inserir um caractere a string
    _hello.push('w');

    println!("{}", _hello);

    //inserir uma string diretamente
    _hello.push_str("old");

    println!("{}", _hello);

    //obtendo a capacidade
    println!("{}", _hello.capacity());

    //verificando se esta vazia
    println!("{}", _hello.is_empty()); // saida: false

    //verifica se a string contém os caracteres
    println!("{}", _hello.contains("wold")); // saida: true

    //substituido um conjunto de caracteres por outro
    println!("{}", _hello.replace("hello", "Hi"));

    //loop obtendo as palavras da frase
    //por meio do espaço em branco

    let mut _frase = String::from("Rust: Uma linguagem capacitando todos a construir softwares confiáveis e eficientes");
    
    for palavra in _frase.split(' ')
    {
        //imprime a palavra "capturada" assim que encontra um espaço em brando
        println!("{}", palavra);
    }

    //criando uma string com capacidade maxima definida

    let mut _s = String::with_capacity(10);

    _s.push('h');
    _s.push_str("ello");

    println!("{}", _s);
    
    let mut _ss = String::default();
    
    println!("{}", _ss.capacity()); // saida: 0. não foi inicializada com nada. porem é mutavel;
}