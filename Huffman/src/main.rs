/// Count the frequencies of all alphabet letters in a text
// NOTE(elsuizo:2018-03-21):esta implementacion la encontre en el siguiente blog:http://pramode.in/2016/09/26/huffman-coding-in-rust/#representing-the-tree-node-in-c


// TODO(elsuizo:2018-03-21):primero lo leemos de aca luego habria que hacer una funcion que
// lea un txt externo con manejo de errores y ...etc
const STRING: &'static str = "El hardware es lo que hace a una máquina rápida; el software es lo que hace que una máquina rápida se vuelva lenta";

// NOTE(elsuizo:) Esta implementacion tiene una magia con el metodo o funcion or_inser()
// Ensures a value is in the entry by inserting the default if empty, and returns a mutable reference to the value in the entry.
//
use std::collections::HashMap;

fn freqs(input: &str) -> HashMap<char, i64> {
    let mut dictionary = HashMap::new();
    for c in input.chars() {
        let counter = dictionary.entry(c).or_insert(0);
        *counter += 1;
    }
    dictionary
}

// NOTE(elsuizo:2018-03-21):esta seria la version con un Option como type de retorno
//
// fn freqs2(input: &str) -> Option<HashMap<char, i64>>{
//     let mut dictionary = HashMap::new();
//     for c in input.chars() {
//         let counter = dictionary.entry(c).or_insert(0);
//         *counter += 1;
//         println!("counter: {:}", counter);
//     }
//     Some(dictionary)
// }

// NOTE(elsuizo:2018-03-21):una vez que tenemos las probabilidades creamos el arbol de
// Huffman, como no sabemos a priori cuantos nodos va a tener el arbol, tenemos que allocar
// memoria dinamicamente, en Rust lo hacemos con la palabra reservada con el type Box<>
// en C la implementacion de un arbol binario seria:
// struct Node {
//  int ch;
//  int freq;
//  struct node* left;
//  struct node* right;
// };
// Pero cual es el mecanismo para representar el NULL en Rust??? ---> el type Option<>
// el cual podemos tener un valor(Some()) o nada(None).
#[derive(Debug)]
struct Node {
    freq: i32,
    ch: Option<char>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
// NOTE(elsuizo:2018-03-21):entonces podemo crear un arbol binario de la siguiente manera:
// let mut p = Box::new(Node{
//                      freq:10,
//                      ch: Some('A')
//                      left: None,
//                      right: None,});
// let mut q = Box::new(Node{
//                      freq:37,
//                      ch: Some('m')
//                      left: None,
//                      right: None,});

// NOTE(elsuizo:2018-03-21): Dada un HashMap el cual mapea cada elemento de la fuente con su
// correspondiente frecuencia es facil generar un vector donde cada elemento de este es un puntero
// a un Node

fn new_node(freq: i32, ch: Option<char>) -> Node {
    Node{
        freq: freq,
        ch: ch,
        left: None,
        right: None,
    }
}

fn new_Box_to_Node(n: Node) -> Box<Node> {
    Box::new(n)
}

fn generate_code_huffman(p: &Box<Node>, h: &mut HashMap<char, String>, s: String) {
    if let Some(ch) = p.ch {
        h.insert(ch, s);
    } else {
        if let Some(ref l) = p.left {
            generate_code_huffman(l, h, (s.clone() + "0"));
        }
        if let Some(ref r) = p.right {
            generate_code_huffman(r, h, (s.clone() + "1"));
        }
    }
}

fn encode_string(s: &str, h: &HashMap<char, String>) -> String {
    let mut r = "".to_string();
    let mut t:Option<&String>;
    for ch in s.chars() {
        t = h.get(&ch);
        r.push_str(t.unwrap());
    }
    r
}

fn decode_strings(s: &str, root: &Box<Node>) -> String {
    let mut result = "".to_string();
    let mut node_ptr = root;

    for x in s.chars() {
        if x == '0' {
            if let Some(ref l) = node_ptr.left {
                node_ptr = l;
            }
        } else {
            if let Some(ref r) = node_ptr.right {
                node_ptr = r;
            }
        }
        if let Some(ch) = node_ptr.ch {
            result.push(ch);
            node_ptr = root;
        }
    }
    result
}

fn main() {
    // primero calculamos las frecuencias
    let frequencies = freqs(STRING);
    // construimos un arbol binario con las frecuencias
    let mut p: Vec<Box<Node>> = frequencies
        .iter()
        .map(|x| new_Box_to_Node(new_node(*(x.1) as i32, Some(*(x.0)))))
        .collect();

    // construimos el arbol de Huffman
    // NOTE(elsuizo:2018-03-21): la mejor estructura de datos que podemos utilizar aca es
    // que volver a ordenar nuevamente los elementos a cada iteracion
    while p.len() > 1 {
        p.sort_by(|a, b| (&(b.freq)).cmp(&(a.freq))); // comparing the elements and sorted
        let a = p.pop().unwrap();
        let b = p.pop().unwrap();
        let mut c = new_Box_to_Node(new_node(a.freq + b.freq, None));
        c.left = Some(a);
        c.right = Some(b);
        p.push(c);
    }

    let root = p.pop().unwrap();
    println!("root = {:?}", root);
    let mut h: HashMap<char, String> = HashMap::new();

    generate_code_huffman(&root, &mut h, "".to_string());
    let enc = encode_string(STRING, &h);
    println!("encode = {:?}", enc);
    println!("decoded = {:?}", decode_strings(&enc, &root));

}

