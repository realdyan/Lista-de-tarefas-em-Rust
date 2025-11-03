use std::io;

struct Tarefa {
    
    //data_tarefa: String,
    //horario_tarefa: String,
    //obs: String,
}

fn criar_tarefa() {
    
    // println!("Data da tarefa:");
    //println!("Horário da tarefa:");

    let mut tarefa: Vec<String> = Vec::new();

    println!("Tarefa:"); // Pedir entrada de dados do usuario

    io::stdin()
    .read_line(&mut tarefa)
    .expect("Error")
    .push(tarefa.trim().to_string())

    

  //  println!("Obs:");
    
}

fn main() {
    println!("Lista de tarefas");
    println!("");
    
    // Menu lista de tarefas
    println!("Escolha uma opção");
    println!("'1'. Criar tarefa");
    println!("'2'. Visualizar Lista de tarefas");
    println!("'3'. Editar tarefa");
    println!("'4'. Sair");
    
    let mut escolha = String::new();  // ✅ Ponto e vírgula
    
    io::stdin()     
        .read_line(&mut escolha)
        .expect("Error");

    // Converter para número
    let escolha: u32 = escolha
    .trim()
    .parse()
    .expect("Digite um número válido!");

    match escolha {
        1 => criar_tarefa(),
        2 => println!("Escolheu 2!"),
        3 => println!("Escolheu 3!"),
        4 => println!("Saindo!"),
        _ => println!("Resposta errada!"),
    }

}