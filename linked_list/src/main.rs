use aluno::*;
use linked_list::*;
use std::fmt::{Display, Formatter, Result};
use std::io::Write;
use std::process::Command;
use std::result;

fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .status()
            .expect("Falha ao limpar o terminal");
    } else {
        Command::new("clear")
            .status()
            .expect("Falha ao limpar o terminal");
    }
}

fn ler_campo() -> result::Result<String, String> {
    let mut campo = String::new();
    match std::io::stdin().read_line(&mut campo) {
        Err(_) => return Err(String::from("Erro ao ler texto")),
        _ => {}
    }
    Ok(campo.trim().to_string())
}

fn inserir_aluno(lista: &mut LinkedList<Aluno>) -> result::Result<(), String> {
    let nomes_campos = vec!["Nome", "Telefone", "Endereço", "Período atual", "Matrícula"];
    let mut nome = "".to_string();
    let mut telefone = "".to_string();
    let mut endr = "".to_string();
    let mut matr = "".to_string();
    let mut periodo = 1;

    println!("Digite as informações do Aluno a ser inserido");
    for i in 0..=4usize {
        print!("{}: ", nomes_campos[i]);
        std::io::stdout().flush().unwrap();
        let buffer = ler_campo()?;

        match i {
            0 => nome = buffer,
            1 => telefone = buffer,
            2 => endr = buffer,
            3 => {
                let value: u8 = match buffer.parse() {
                    Ok(v) => v,
                    Err(_) => {
                        return Err(String::from(
                            "Valor inválido. Caractere não é número ou o valor é negativo/maior que 255",
                        ));
                    }
                };
                periodo = value;
            }
            4 => matr = buffer,
            _ => {}
        }
    }

    let aluno = Aluno::new(nome, telefone, endr, matr, periodo);
    Edit::push(lista, aluno)
}

fn retirar_aluno(lista: &mut LinkedList<Aluno>) -> result::Result<(), String> {
    print!("Matrícula: ");
    std::io::stdout().flush().unwrap();
    Edit::remove(lista, &ler_campo()?)
}

fn imprimir_lista(lista: &LinkedList<Aluno>) {
    println!("{:-^35}", "Lista de alunos cadastrados");
    lista.print();
}

fn buscar_aluno<'a>(lista: &'a mut LinkedList<Aluno>) -> result::Result<&'a mut Aluno, String> {
    print!("Matrícula: ");
    std::io::stdout().flush().unwrap();
    let campo = ler_campo()?;
    Edit::find(lista, &campo)
}

fn alterar_dados(lista: &mut LinkedList<Aluno>) -> std::result::Result<(), String> {
    print!("Matrícula: ");
    std::io::stdout().flush().unwrap();
    lista.change(&ler_campo()?)
}

fn main() {
    let matricula = String::from("20203014526");
    let aluno1 = Aluno::new(
        "Lucas".to_string(),
        "+55 (31) ...".to_string(),
        "Casa".to_string(),
        matricula,
        12,
    );

    let aluno2 = Aluno::new(
        "Gustavo".to_string(),
        "+55 (21) ...".to_string(),
        "Apartamento".to_string(),
        "20203014525".to_string(),
        3,
    );
    let mut lista = LinkedList::new();

    let _ = Edit::push(&mut lista, aluno1);

    let _ = Edit::push(&mut lista, aluno2);

    println!("Bem-vindo ao sistema de Registro de alunos. Escolha uma opção para começar.");
    loop {
        println!("1. Listar alunos cadastrados");
        println!("2. Mostrar detalhes de um aluno");
        println!("3. Adicionar aluno");
        println!("4. Remover aluno");
        println!("5. Editar registro de aluno");
        println!("q. Sair do programa");

        let buffer = ler_campo();

        let buffer = if let Ok(campo) = buffer {
            &campo.to_lowercase() as &str
        } else {
            panic!("Erro ao ler texto")
        };

        match buffer {
            "1" => {
                clear_terminal();
                imprimir_lista(&lista)
            }
            "2" => {
                if let Ok(encontrado) = buscar_aluno(&mut lista) {
                    clear_terminal();
                    println!("{:-^18}\n\n{}", "Detalhes", encontrado);
                } else {
                    println!("Não encontrado na base de dados");
                }
            }
            "3" => {
                clear_terminal();
                if let Err(e) = inserir_aluno(&mut lista) {
                    println!("{}", e);
                } else {
                    println!("\nSucesso!");
                }
            }
            "4" => {
                if let Err(e) = retirar_aluno(&mut lista) {
                    println!("{}", e);
                } else {
                    println!("\nSucesso!");
                }
            }
            "5" => {
                if let Err(e) = alterar_dados(&mut lista) {
                    println!("{}", e);
                } else {
                    println!("\nSucesso!");
                }
            }
            "q" => break,
            _ => println!("Opção inválida"),
        }

        println!("\nPressione Enter para continuar...");
        match ler_campo() {
            Ok(_) => {}
            Err(_) => panic!("Erro ao ler o texto"),
        };
        clear_terminal();
        println!("------Menu inicial------\n");
        println!("Selecione uma das opções:");
    }
}

mod aluno {
    use super::*;
    #[derive(Debug, Clone)]
    pub struct Aluno {
        nome: String,
        telefone: String,
        endr: String,
        matr: String,
        periodo: u8,
    }

    impl Aluno {
        pub fn new(
            nome: String,
            telefone: String,
            endr: String,
            matr: String,
            periodo: u8,
        ) -> Aluno {
            Aluno {
                nome: nome.trim().to_string(),
                telefone: telefone.trim().to_string(),
                endr: endr.trim().to_string(),
                matr: matr.trim().to_string(),
                periodo,
            }
        }
    }

    fn are_you_sure() -> result::Result<(), String> {
        println!(
            "Excluir aluno da base de dados? (Operação não pode ser desfeita)\ns - Sim\nn - Cancelar"
        );
        loop {
            let mut buffer = String::new();
            if std::io::stdin().read_line(&mut buffer).is_err() {
                return Err(String::from("Erro ao ler texto inserido"));
            };
            buffer = buffer.trim().to_lowercase();

            match &buffer as &str {
                "s" => return Ok(()),
                "n" => return Err(String::from("Operação cancelada")),
                _ => {
                    println!("Opção inválida");
                    continue;
                }
            }
        }
    }

    impl Edit for LinkedList<Aluno> {
        type Value = Aluno;
        type Entry = String;
        type Node = Node<Self::Value>;

        fn push(&mut self, value: Self::Value) -> std::result::Result<(), Self::Entry> {
            match self.head.as_mut() {
                None => {
                    self.push(value);
                    return Ok(());
                }
                Some(item) => {
                    let mut aluno = item;
                    while &aluno.value.matr != &value.matr {
                        if let Some(ref mut seguinte) = aluno.next {
                            aluno = seguinte;
                        } else {
                            aluno.push(value);
                            return Ok(());
                        }
                    }
                    return Err(String::from("Aluno já existe"));
                }
            }
        }

        fn change(&mut self, entry: &str) -> std::result::Result<(), Self::Entry> {
            if let Ok(aluno) = self.find(&*entry) {
                clear_terminal();
                println!("{:-^18}\n\n{}\n", "Detalhes", aluno);
                println!(
                    "O que deseja alterar?\n1. O nome\n2. Número de Telefone\n3. O endereço\n4. Período atual do aluno\nq. Voltar"
                );
                let campo = loop {
                    let mut opcao = String::new();
                    opcao = if std::io::stdin().read_line(&mut opcao).is_err() {
                        return Err(String::from("Erro ao ler texto inserido"));
                    } else {
                        opcao.trim().to_lowercase()
                    };

                    match &opcao as &str {
                        "q" => {
                            return Err(String::from(""));
                        }
                        "1" | "2" | "3" | "4" => break opcao,
                        _ => {
                            println!("Opção inválida");
                            continue;
                        }
                    }
                };

                let campo = campo.as_str();

                println!("Digite o texto para alteração no campo:");

                let mut buffer = String::new();
                if std::io::stdin().read_line(&mut buffer).is_err() {
                    return Err(String::from("Erro ao ler texto inserido"));
                };
                buffer = buffer.trim().to_string();

                loop {
                    match campo {
                        "1" => aluno.nome = buffer,
                        "2" => aluno.telefone = buffer,
                        "3" => aluno.endr = buffer,
                        "4" => {
                            let value: u8 = match buffer.parse() {
                                Ok(v) => v,
                                Err(_) => {
                                    return Err(String::from(
                                        "Valor inválido. Caractere não é número ou o valor é negativo/maior que 255",
                                    ));
                                }
                            };
                            aluno.periodo = value;
                        }
                        _ => return Err(String::from("Opção inválida")),
                    }
                    return Ok(());
                }
            }
            Err(String::from("Aluno não encontrado"))
        }

        fn remove(&mut self, entry: &str) -> std::result::Result<(), Self::Entry> {
            // Conferir o primeiro aluno da lista:
            // Deve ser feita a conferência separadamente do resto da lista devido às.
            if self.head.is_some() && &self.head.as_ref().unwrap().value.matr == entry {
                clear_terminal();
                println!(
                    "{:-^24}\n\n{}\n",
                    "Aluno encontrado",
                    self.head.as_ref().unwrap().value
                );
                are_you_sure()?;
                let node_to_remove = self.head.take().unwrap();
                self.head = node_to_remove.next;
                return Ok(());
            }

            let mut cursor = &mut self.head;

            // Loop enquanto o cursor não for None
            // como 'cursor' é Option<Box<Node<Aluno>>>
            // 'node' será &mut Box<Node<Aluno>>
            while let Some(node) = cursor {
                // 2. Conferir se o próximo aluno é o que deve ser removido
                // É feito assim porque o ponteiro do cursor deve ser usado para alterar o ponteiro do próximo aluno
                if let Some(prox) = node.next.as_mut() {
                    if &prox.value.matr == entry {
                        // O aluno prox será removido (node.next)
                        clear_terminal();
                        println!(
                            "{:-^24}\n\n{}\n",
                            "Aluno encontrado",
                            node.next.as_ref().unwrap().value
                        );

                        are_you_sure()?;

                        // É tomada a posse do conteúdo de prox.
                        // prox é colocado em None temporariamente.
                        let aluno_retirado = node.next.take().unwrap();

                        // A lista é ligada novamenet
                        // O ponteiro de cursor passa a apontar para o nó depois do que foi retirado
                        node.next = aluno_retirado.next;

                        return Ok(());
                    }
                }

                // Se o aluno ainda não foi encontrado, mover o cursor para o próximo aluno
                cursor = &mut node.next;
            }

            // Se sair do loop (cursor == None), retornar o erro de que o aluno não foi encontrado.
            return Err(String::from("Aluno não encontrado"));
        }

        fn find(&mut self, entry: &str) -> result::Result<&mut Self::Value, String> {
            match &mut self.head {
                None => return Err(String::from("Lista está vazia")),
                Some(item) => {
                    let mut aluno = item;
                    while &aluno.value.matr != entry {
                        if let Some(ref mut seguinte) = aluno.next {
                            aluno = seguinte;
                        } else {
                            return Err(String::from("Aluno não encontrado na base de dados"));
                        }
                    }
                    Ok(&mut aluno.value)
                }
            }
        }

        fn print(&self) {
            let mut registro = &self.head;
            loop {
                match registro {
                    Some(aluno) => {
                        println!("{}\n", aluno.value);
                        registro = &aluno.next;
                    }
                    None => break,
                }
            }
        }
    }

    impl Display for Aluno {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "Matrícula: {}\nNome: {}\nTelefone: {}\nEndereço: {}\nPeríodo atual: {}",
                self.matr, self.nome, self.telefone, self.endr, self.periodo
            )
        }
    }
}

mod linked_list {
    use super::*;

    pub struct LinkedList<T> {
        pub head: Option<Box<Node<T>>>,
    }

    #[derive(Debug, Clone)]
    pub struct Node<T> {
        pub value: T,
        pub next: Option<Box<Node<T>>>,
    }

    pub trait Edit {
        type Value;
        type Entry;
        type Node;

        fn push(&mut self, value: Self::Value) -> std::result::Result<(), Self::Entry>;
        fn change(&mut self, entry: &str) -> std::result::Result<(), Self::Entry>;
        fn remove(&mut self, entry: &str) -> std::result::Result<(), Self::Entry>;
        fn find(&mut self, entry: &str) -> std::result::Result<&mut Self::Value, Self::Entry>;
        fn print(&self);
    }

    impl<T> LinkedList<T> {
        pub fn new() -> Self {
            Self { head: None }
        }

        pub fn push(&mut self, value: T) {
            match &mut self.head {
                None => self.head = Some(Box::new(Node::new(value))),
                Some(node) => node.push(value),
            }
        }
    }

    impl<T> Node<T> {
        pub fn new(value: T) -> Self {
            Self { value, next: None }
        }

        pub fn push(&mut self, value: T) {
            let mut next_item = self;
            loop {
                if let Some(ref mut item) = next_item.next {
                    next_item = item;
                } else {
                    next_item.next = Some(Box::new(Node::new(value)));
                    break;
                }
            }
        }
    }

    impl<T: Display> Display for LinkedList<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match &self.head {
                Some(node) => write!(f, "{}", node),
                None => write!(f, "[]"),
            }
        }
    }

    impl<T: Display> Display for Node<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let mut item = self;
            write!(f, "[{}", item.value)?;

            while let Some(next) = &item.next {
                write!(f, ", {}", next.value)?;
                item = next;
            }

            write!(f, "]")
        }
    }
}
