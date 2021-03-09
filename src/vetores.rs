//vetors são arrays dinamicos

use std::mem;

pub fn run()
{
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    println!("{:?}", numbers);

    //obter um unico valor
    println!("{}", numbers[1]);

    let mut _numbers:[i32; 5] = [1,2,3,4,5];
    _numbers[4] = 20;

    println!("{:?}", _numbers);

    println!("numero de elementos: {}", _numbers.len());
    println!("numero em bytes: {}", mem::size_of_val(&_numbers));

    
    let arr: &[i32] = &_numbers;

    println!("{:?}", arr);

    //obter partes de outro array com copia

    let arr2: &[i32] = &_numbers[2..5];

    println!("{:?}", arr2);


    //modificando vectors

    //adicinando elementos ao vetor;

    numbers.push(35);
    numbers.push(40);

    println!("{:?}", numbers);

    //remove o ultimo valor (ideia de pilha)
    numbers.pop();

    println!("{:?}", numbers);

    //loops

    //itera cada elemento do vetor
    for x in numbers.iter()
    {
        println!("{}", x);
    }

    //alterar o proprio vetor
    for x in numbers.iter_mut()
    {
        *x = *x * 2; //x é uma variavel temporaria. eme seguida o seu valor é adicionado ao proprio vetor

        // "x" é um ponteiro para o vetor numbers obtido a cada iteração atraves do metodo "iter_mut()"
    }

    println!("{:?}", numbers);

    for x in numbers.iter_mut()
    {
        *x = *x - 2; 
    }

    println!("{:?}", numbers);

}