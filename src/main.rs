/*  para executar, rode na pasta do projeto: "cargo run"
    por exemplo, navegue ate a pasta do projeto:
    ~/Documentos/Rust/rust_exemples$ 
*/

// documentação oficial: https://doc.rust-lang.org/book/title-page.html
// biblioteca padrão: https://doc.rust-lang.org/std/index.html

//importa o arquivo chamado "saida" 
//mod saida;
//mod tipo_de_dados;
//mod strings;
//mod condicionais;
//mod tuplas;
//mod arrays;
//mod vetores;
//mod loops;
//mod funcoes;
mod structs;

//importa a função do arquivo diretamente
//use saida::run;

fn main() {
    //executa a função run() importada do arquivo "saida.rs"
    //saida::run();

    //tipo_de_dados::run();
    //strings::run();
    //condicionais::run();
    //tuplas::run();
    //arrays::run();
    //vetores::run();
    //loops::run();

    /* 
    funcoes::run();
    let s = funcoes::soma(5, 5);
    println!("função soma retorno: {}", s);
    */

    structs::run();

}
