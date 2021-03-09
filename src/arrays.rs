use std::mem;

pub fn run()
{
    //tipo explicito não mutavel
    let numbers:[i32; 5] = [1,2,3,4,5];


    println!("{:?}", numbers);

    //obter um unico valor
    println!("{}", numbers[1]);

    //tipo explicito mutavel
    let mut _numbers:[i32; 5] = [1,2,3,4,5];
    _numbers[4] = 20;

    println!("{:?}", _numbers);

    //obtem o numero de elementos do array
    println!("numero de elementos: {}", _numbers.len());

    //obtem o tamanho em bytes
    println!("numero em bytes: {}", mem::size_of_val(&_numbers));
    
    // "Copia" de outro vetor
    let arr: &[i32] = &_numbers;

    println!("{:?}", arr);

    //obter partes de outro array com copia

    let arr2: &[i32] = &_numbers[2..5];

    println!("{:?}", arr2);

}