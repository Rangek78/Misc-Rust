use aluno::*;
use linked_list::LinkedList;

fn main() {
    let aluno = Aluno::new(
        "Lucas".to_string(),
        "31991820082".to_string(),
        "Casa".to_string(),
        "20203014526".to_string(),
        12,
    );

    let aluno2 = Aluno::new(
        "Gustavo".to_string(),
        "number".to_string(),
        "Apartamento".to_string(),
        "20203014525".to_string(),
        3,
    );
    let mut list = LinkedList::new(aluno);

    inserir_aluno(&mut list, aluno2);

    println!("{:?}", list);
}

mod aluno {
    use crate::linked_list::LinkedList;
    #[derive(Debug)]
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

    pub fn buscar_aluno<'a>(
        lista: &'a mut LinkedList<Aluno>,
        matricula: &String,
    ) -> Option<&'a Aluno> {
        let mut dados = lista;
        while &dados.value.matr != matricula {
            if let Some(ref mut seguinte) = dados.next {
                dados = seguinte;
            } else {
                return None;
            }
        }
        Some(&dados.value)
    }

    pub fn inserir_aluno(lista: &mut LinkedList<Aluno>, aluno: Aluno) {
        if let Some(_) = buscar_aluno(lista, &aluno.matr) {
            println!("Aluno já existe");
            return;
        }
        let mut dados = lista;
        while let Some(ref mut seguinte) = dados.next {
            dados = seguinte;
        }
        dados.push(aluno);
    }
}

mod linked_list {
    use std::fmt::{Display, Formatter, Result};
    #[derive(Debug)]
    pub struct LinkedList<T> {
        pub value: T,
        pub next: Option<Box<LinkedList<T>>>,
    }

    impl<T> LinkedList<T> {
        pub fn new(value: T) -> Self {
            Self {
                value: value,
                next: None,
            }
        }

        pub fn push(&mut self, value: T) {
            let mut next_item = self;
            loop {
                if let Some(ref mut item) = next_item.next {
                    next_item = item;
                } else {
                    next_item.next = Some(Box::new(LinkedList::new(value)));
                    break;
                }
            }
        }
        /*
        pub fn next(&mut self) -> &mut Option<Box<LinkedList<T>>> {
            &mut self.next
        }

        pub fn value(&self) -> &T {
            &self.value
        }
        */
    }

    impl<T: Display> Display for LinkedList<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let mut item = self;
            write!(f, "[{}", item.value)?;

            while let Some(next) = &item.next {
                write!(f, ", {}", next.value)?;
                item = next;
            }

            write!(f, "]")?;
            Ok(())
        }
    }
}
