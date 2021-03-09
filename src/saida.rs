// os ememplos a seguir, mostram as maneiras
//de como exibir saida de dados no console


//função publica ("pub") chamada "run"
pub fn run()
{
    //comando de saida comum
    println!("Olá mundo !!");

    //saida formatada
    //cada "chave" escrita, delimita onde cada conteúdo deve ser escrito definido após a virgula
    println!("{}, {} e {} são exelentes linguagens de programação", "Rust", "C/C++", "C#");

    //argumentos nomeados
    println!("{linguagem1} {linguagem2} {linguagem3}", linguagem2 = "C++", linguagem1 = "Rust", linguagem3 = "C#");

    //formatações de saida
    println!("numero 20 em Binario: {:b}, Hexadecimal: {:x}, Octal: {:o}", 20,20,20);

    //formatação rapida para depuração

    println!("{:?}", ("rust", "é", true, 10));

    //calculos diretamente na saida

    println!("10 + 10 é igual a {}", 10 + 10);
}