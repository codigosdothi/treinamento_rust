fn main() {
/* 
Dado que eu tenha um ano de nascimento, e faço a subtração pelo ano atual, 
Então devo obter a idade do indíviduo.
 */

 let nome = "Thiago";
 let dia_nascimento = 21;
 let mes_nascimento = 5;
 let ano_nascimento = 1986;
 let dia_atual = 13;
 let mes_atual = 8;
 let ano_atual = 2026;


 let mut idade = ano_atual - ano_nascimento;
 if mes_nascimento > mes_atual {
     idade -= 1;
 }
 else if dia_nascimento > dia_atual && mes_nascimento == mes_atual {
     idade -= 1;
 }
    println!("Hello World, A idade do {} calculada para o ano de nascimento {} é de: {}", nome, ano_nascimento, idade);
}
