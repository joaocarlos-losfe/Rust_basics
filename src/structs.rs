//são similares as classes

//struct comum

struct Cores
{
    red: u8, //inteiros de 0 a 255
    green: u8,
    blue: u8,
}

// struct do tipo tupla

struct Stuple(u8, u8, u8);

//struct do tipo classe

//----------
struct Pessoa
{
    primeiro_nome : String,
    sobrenome : String
}

impl Pessoa
{
    //"construtor"
    fn nova_pessoa(primero: &str, ultimo: &str) -> Pessoa
    {
        Pessoa
        {
            primeiro_nome : primero.to_string(),
            sobrenome: ultimo.to_string()
        }
    }

    //retorna o nome completo
    fn nome_completo(&self) -> String
    {
        format!("{} {}", self.primeiro_nome, self.sobrenome)
    }

    fn set_nome_sobrenome(&mut self, nome: &str, sobrenome: &str)
    {
        self.primeiro_nome = nome.to_string();
        self.sobrenome = sobrenome.to_string();
    }

}
//------------


pub fn run()
{
    let mut _cor = Cores{
        red : 10,
        green : 5,
        blue : 255
    };

    println!("{:?}", (_cor.red, _cor.green, _cor.blue));

    _cor.blue = 20;

    println!("{:?}", (_cor.red, _cor.green, _cor.blue));

    //------------------------------------

    let mut _s = Stuple(1,2,3);

    println!("{:?}", (_s.0, _s.1, _s.2));

    _s.2 = 5;

    println!("{:?}", (_s.0, _s.1, _s.2));

    // ---------------------------------


    let mut _pessoa = Pessoa::nova_pessoa("joão carlos", "Sousa Fé");

    println!("{}", _pessoa.nome_completo());

    _pessoa.set_nome_sobrenome("jose", "sousa");

    println!("{:?}", (_pessoa.primeiro_nome, _pessoa.sobrenome));

    
}