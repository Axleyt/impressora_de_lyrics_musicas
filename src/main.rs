use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let listas_de_musica = [
        "Hey jude - The Beatles",
        "She's a big boy - McBaise",
        "teste",
    ];
    loop {
        let escolha_musica = loop {
            //Input de escolha do user de qual musica irá querer ver a letra
            println!("Por favor escolha uma das musicas abaixo:\n");

            for (indice, mostrando_catalago) in listas_de_musica.iter().enumerate() {
                println!("{} - {}", indice + 1, mostrando_catalago);
            }
            println!("\nSe deseja sair digite: 0");

            let mut escolha = String::new();
            io::stdin()
                .read_line(&mut escolha)
                .expect("Erro em ler a mensagem");

            match escolha.trim() {
                "1" | "2" | "3" => break escolha,
                "0" => {
                    clearscreen::clear().expect("erro em limpar a tela");
                    print!("Tem certeza? [S/N]: ");
                    io::stdout().flush().expect("falha a mostrar mensagem");

                    let mut confirmacao_de_saida: String = String::new();
                    io::stdin()
                        .read_line(&mut confirmacao_de_saida)
                        .expect("Falha a ler a confirmção de saida");

                    match confirmacao_de_saida.trim().to_uppercase().as_str() {
                        "S" | "SIM" => std::process::exit(0),

                        "N" | "NAO" | "NÃO" => {
                            clearscreen::clear().expect("falha a limpar a tela");
                        }

                        _ => {
                            clearscreen::clear().expect("falha a limpar a tela");
                            println!("Por favor escolha entre SIM ou NÃO");
                        }
                    }
                }
                _ => {
                    println!("opção invalida, por favor selecione alguma valida");
                }
            }
        };
        clearscreen::clear().expect("erro em limpar a tela");
        match escolha_musica.trim() {
            "1" => {
                println!("{}\n", listas_de_musica[0]);
                impressora_das_letras(hey_jude());
            }
            "2" => {
                println!("{}\n", listas_de_musica[1]);
                impressora_das_letras(she_big_boy());
            }
            "3" => {
                println!("{}\n", listas_de_musica[2]);
                impressora_das_letras(teste());
            }
            _ => {
                println!("Por favor selecione alguma música");
            }
        }
        //Esperando o user confirmar
        pausa_do_programa();
    }
}

fn impressora_das_letras(x: &str) {
    //1°For: serve para trazer o verso da musica
    for versos in x.split('\n') {
        if versos.trim().is_empty() {
            println!();
            thread::sleep(Duration::from_millis(500));
            continue;
        }
        //2° for: captuara cada palavra do verso e mostra ela com um delay
        for palavras in versos.trim().split(" ") {
            print!("{} ", palavras);
            io::stdout().flush().expect("Erro em mostrar a imagem");
            thread::sleep(Duration::from_millis(300));
        }
        println!();
        thread::sleep(Duration::from_millis(500));
    }
}

fn pausa_do_programa() {
    print!("\nPressione ENTER para ver outras letras musicais....");
    io::stdout().flush().expect("falha ao exibir a mensagem");

    let mut pausa = String::new();
    io::stdin()
        .read_line(&mut pausa)
        .expect("erro em ler o input");

    clearscreen::clear().expect("erro em limpar a tela")
}

fn teste() -> &'static str {
    "tesntando tesntando tesntando 
    tesntando tesntando tesntando 
    tesntando tesntando tesntando 
    
    tesntando tesntando tesntando 
    tesntando tesntando "
}

fn hey_jude() -> &'static str {
    "Hey, Jude, don't make it bad
    Take a sad song and make it better
    Remember to let her into your heart
    Then you can start to make it better

    Hey, Jude, don't be afraid
    You were made to go out and get her
    The minute you let her under your skin
    Then you begin to make it better

    And anytime you feel the pain
    Hey, Jude, refrain
    Don't carry the world upon your shoulders

    For well you know that it's a fool
    Who plays it cool
    By making his world a little colder
    Na-na-na-na-na-na-na-na

    Hey, Jude, don't let me down
    You have found her, now go and get her
    Remember to let her into your heart
    Then you can start to make it better

    So let it out and let it in
    Hey, Jude, begin
    You're waiting for someone to perform with
    And don't you know that is just you?
    Hey, Jude, you'll do
    The movement you need is on your shoulder
    Na-na-na-na-na-na-na-na

    Hey, Jude, don't make it bad
    Take a sad song and make it better
    Remember to let her under your skin
    Then you'll begin to make it better
    (Better, better, better, better, better, oh, yeah)

    Na, na-na-na-na-na-na-na-na, hey, Jude
    Na, na-na-na-na-na-na-na-na, hey, Jude
    Na, na-na-na-na-na-na-na-na, hey, Jude
    Na, na-na-na-na-na-na-na-na, hey, Jude"
}

fn she_big_boy() -> &'static str {
    "Ela É Um Menino Grande Agora
    She's a Big Boy

    Vamos brincar de cowboy
    Let's play cowboy

    Brinquedo de sete polegadas
    Seven-inch toy

    No seu cinto
    At your belt

    Através do vapor
    Through the vapor

    Veja o interior da cabeça dela
    See the inside of her head

    Ela é um menino grande agora
    She's a big boy now

    Ela é um meninão
    She's a big boy

    Ela tem o cabelo e os sapatos amarrados em forma forca
    She's got her hair and her shoes all tied up in a noose

    Ela é um menino grande agora
    She's a big boy now

    Ela tem o cabelo e os sapatos amarrados em forma forca
    She's got her hair and her shoes all tied up in a noose

    Você pode, por favor, me dizer
    Can you please tell me

    O que há embaixo da minha cama?
    What's underneath my bed?

    Ah, os monstros
    Oh, the monsters

    Eles não estão apenas dentro da minha cabeça
    They ain't just inside my head

    Ela é um menino grande agora
    She's a big boy now

    Ela é um meninão
    She's a big boy

    Ela tem o cabelo e os sapatos amarrados em forma forca
    She's got her hair and her shoes all tied up in a noose

    Ela é um menino grande agora
    She's a big boy now

    Ela tem o cabelo e os sapatos amarrados em forma forca
    She's got her hair and her shoes all tied up in a noose
    "
}
