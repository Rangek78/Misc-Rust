use std::cmp::Ordering;

pub struct BinTree<T: Ord> {
    value: T,                       // valor no nó da árvore
    left: Option<Box<BinTree<T>>>,  // struct à esquerda do nó da árvore
    right: Option<Box<BinTree<T>>>, // struct à direita do nó da árvore
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

    fn insert_iter(&mut self, value: T) {
        let mut node = self;
        loop {
            match value.cmp(&node.value) {
                // se o valor a ser inserido é menor que o nó sendo analisado
                Ordering::Less => {
                    if let Some(ref mut left_node) = node.left {
                        node = left_node;
                    } else {
                        node.left = Some(Box::new(BinTree::new(value)));
                        break;
                    }
                }
                // se o valor a ser inserido é maior que o nó sendo analisado
                Ordering::Greater => {
                    if let Some(ref mut right_node) = node.right {
                        node = right_node;
                    } else {
                        node.right = Some(Box::new(BinTree::new(value)));
                        break;
                    }
                }
                // se o valor a ser inserido for igual ao do nó sendo analisado, pular inserção
                Ordering::Equal => break,
            }
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

    fn numel_iter(&self) -> usize {
        let mut count = 0;
        let mut nodes = vec![self];

        while let Some(node) = nodes.pop() {
            if let Some(ref left) = node.left {
                nodes.push(left);
            }
            if let Some(ref right) = node.right {
                nodes.push(right);
            }

            count += 1;
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

    println!("Início: número de elementos na árvore: {}", root.numel());

    root.insert(19);
    root.insert(23);
    root.insert(21);
    root.insert(10);
    root.insert(15);
    root.insert_iter(8);
    root.insert_iter(14);
    root.insert_iter(36);
    root.insert_iter(30);
    root.insert_iter(0);
    root.insert_iter(-10);

    println!("Fim: número de elementos na árvore: {}", root.numel_iter());
}
