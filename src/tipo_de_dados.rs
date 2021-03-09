//tipos primitivos basicos

/*
inteiros
{
    --unsigned
    u8, u16, u32, u64, u128; 

    --signed
    i8, i16, i32, i64, i128
}

floats
{
    f32, f64;
}

booleanos
{
    bool;
}

characters
{
    chars;
}
*/

pub fn run()
{
    //tipo inferido não mutavel: padrão i32
    let x = 1;
    //tipo inferido não mutavel: padrão f64
    let y = 2.5;

    //tipo explicito não mutaveis
    let j: i32 = 10; // inteiro de 32 bits
    let k: i64 = 2834234634; //inteiro de 64 bits
    let z: f32 = 345.23123; // flutuante de 32 bits

    //tipo explicito mutavel.
    //por convenção, variaveis mutaveis são presecida de underscore: _variavel e não precisa serem inicializadas;
    let mut _var1: i32;
    let mut _var2: f64;

    //tipos mutaveis com tipo não inferido, por convensão, devem ser inicializadas com algum valor
    let mut _n1 = 5;
    let mut _nome = "jose";

    println!("{:?}", (_n1, _nome));

    _n1 = 3;
    _nome = "antonio";

    println!("{:?}", (_n1, _nome));

    //como saber o tamanho maximo de um tipo de um tipo;
    println!("max i32: {}", std::i32::MAX); //saida: 2147483647

    println!("max f64: {}", std::f64::MAX); //saida: 79769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000

    //--booleanos

    //implicito
    let is_active = true;

    //explicito mutavel
    let mut _is_save: bool = false;

    //caracteres

    let ch1 =  'a';

    let ch2: char;
    ch2 = 'b';

    let face = '\u{1f600}'; // caratere unicode (emogi)

    println!("{:?}", (x, y, j, k, z, is_active, _is_save, ch1, ch2, face));

}