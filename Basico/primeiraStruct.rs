struct Estudante {
    nome: String,
    nivel: u8,
    remoto: bool
}

struct Series(
    char,
    char,
    char,
    char,
    f32
);

fn main() {
    let aluno_1 = Estudante { nome: String::from("Fulano"), nivel: 2, remoto: true };
    let notas_1 = Series('A', 'A', 'B', 'A', 3.75);

    println!("Aluno:\n----------");
    println!("Nome: {}, Nivel: {}, Remoto: {}", aluno_1.nome, aluno_1.nivel, aluno_1.remoto);

    println!("Notas:\n----------");
    println!("Nota 1: {}, Nota 2: {}, Nota 3: {}, Nota 4: {}", notas_1.0, notas_1.1, notas_1.2, notas_1.3);
    println!("Final: {}", notas_1.4);
}
