use std::cmp::Ordering;

/*
value: valor no nó da árvore
left: struct à esquerda do nó da árvore
right: struct à direita do nó da árvore
*/
pub struct BinTree<T: Ord> {
    value: T,
    left: Option<Box<BinTree<T>>>,
    right: Option<Box<BinTree<T>>>,
}

impl<T: Ord> BinTree<T> {
    // Um nó novo é uma folha. Não existem valores embaixo de uma folha, então são inicializados com None.
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    // Insere um novo nó na árvore de acordo com a estrutura da árvore binária.
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            // se o valor a ser inserido é menor que o nó sendo analisado
            Ordering::Less => match self.left {
                Some(ref mut left_node) => left_node.insert(value),
                None => self.left = Some(Box::new(BinTree::new(value))),
            },
            // se o valor a ser inserido é maior que o nó sendo analisado
            Ordering::Greater => match self.right {
                Some(ref mut right_node) => right_node.insert(value),
                None => self.right = Some(Box::new(BinTree::new(value))),
            },

            // se o valor a ser inserido for igual ao do nó sendo analisado, pular inserção
            Ordering::Equal => {}
        }
    }

    fn numel(&self) -> usize {
        let mut count = 1;

        if let Some(node) = &self.left {
            count += node.numel();
        }
        if let Some(node) = &self.right {
            count += node.numel();
        }

        count
    }
}

fn main() {
    let mut root = BinTree::new(20);

    // let mut buffer = String::new();
    // std::io::stdin().read_line(&mut buffer).unwrap();

    // let value = buffer.trim().parse().unwrap();
    // root.insert(value);

    root.insert(19);
    root.insert(23);
    root.insert(21);
    root.insert(20);
    root.insert(10);
    root.insert(15);
    root.insert(8);
    root.insert(14);
    root.insert(36);
    root.insert(30);
    root.insert(0);
    root.insert(-10);

    println!("{}", root.numel());
}
