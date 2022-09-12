use std::io;

fn main() {
    println!("{}",fibo());
    let mut string : String = String::new();
    io::stdin()
        .read_line(&mut string)
        .expect("Error reading string");
    println!("{}",&string);

    let int_text : i32 = string.trim().parse().expect("Could not parse");

    println!("{}",int_text*2);

    println!("Hello, world! {}",string);
    loop_test();
    ownership_testing();
    test()
}

//Testes de looping

fn loop_test(){
    let mut count : i32 = 0;
    let result = 'loop_label: loop {
        count += 1;

        if count == 10 {
            break 'loop_label count * 2;
        }
    };
    println!("Result be like: {}", result);

}


//Testes de Ownership
//Tipos de string não são previstos na alocação de memoria, então qualquer transição por declaração variaveis ou chamada de função muda o escopo ou inutiliza a variavel antiga,
//causa uma liberação dupla de memoria, o que pode ser uma falha de integridade/segurança na aplicação :)
fn ownership_testing(){
    let s = String::from("AAAA");
    let s1 = ownership_string_gives_and_returns(s);
    ownership_taking(s1);
    //A partir daqui a ownership é transferida e passar a sair desse escopo/libera a memoria

    let i = 9;
    ownership_copy(i);
    ownership_copy(i);
    //A variavel i continuara a existindo, tipos como boolean, chars, floats e integers são copiados e não tem sua ownership transferida por não serem tão custosos
}
//A partir daqui ja era, o escopo das variaveis acima é apagado, logo todas a variaveis saem da memoria

fn ownership_taking(string : String){
    println!("Liberando da memoria a string: {}",string);
}

fn ownership_copy(number : i32){
    println!("Fazendo copia do numero: {}",number)
}
//Aqui retornamos novamente nossa string caso haja necessiade de reaproveitar
fn ownership_string_gives_and_returns(string : String) -> String {
    println!("Utilizando e devolvendo a string: {}",string);
    string
} 

fn test() {
    let mut stringTest : String = String::new();

    std::io::stdin().read_line(&mut stringTest).expect("erro");

    println!("Essa é a mensagem: {}", stringTest);
}

fn fibo() -> i32 {
    let n = 2;
    let mut accA = 0;
    let mut accB = 0;
    let mut accC = 0;
    
    let mut finalAcc = 0;
    let mut it = 0;
    
    loop {
        if it == n {
            break 
        }
        if it == 0 {
            accA = 1;
        } else {
            accC = accB;
            accB = accA;
            accA = accB + accC;
        }
        
        it += 1
    }
    
    accA
}