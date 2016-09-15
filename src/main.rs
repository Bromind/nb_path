use std::collections::HashSet;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::fs::File;
use std::env;

#[derive(Hash, Eq, PartialEq, Clone)]
struct Vertice {
    begin: char,
    end: char,
}

fn main() {

    let mut args = env::args();

    if args.len() < 4 {
        // Safe (always have at least program name)
        println!("Usage: {} b e /path/to/graph/file\n\nb is the starting vertice\ne is the ending vertice", args.nth(0).unwrap());
        return;
    }

    // Safe after check above
    let b: char = args.nth(1).unwrap().pop().unwrap();
    let e: char = args.next().unwrap().pop().unwrap();
    let path = args.next().unwrap();

    let res = read_graph(path);

    match res {
        Ok(graph) => {
            println!("{}", nb_path(e, vec![b], graph));
            return;
        },
        Err(_) => return,
    }
}

fn nb_path(dest: char, path: Vec<char>, graph: HashSet<Vertice>) -> i32 {
    let last = path[path.len() - 1];
    let mut nb: i32 = 0;
    if last != dest {
        for vertice in graph.clone() {
            if vertice.begin == last  {
                let mut new_path = path.clone();
                new_path.push(vertice.end);
                let mut new_graph = graph.clone();
                new_graph.remove(&vertice);
                nb += nb_path(dest, new_path, new_graph);
            }
            if vertice.end == last  {
                let mut new_path = path.clone();
                new_path.push(vertice.begin);
                let mut new_graph = graph.clone();
                new_graph.remove(&vertice);
                nb += nb_path(dest, new_path, new_graph);
            }
        }
    } else {
        nb = 1;
    };
    nb
}

fn read_graph(filename: String) -> Result<HashSet<Vertice>, Error> {
    let f = try!(File::open(filename));
    let reader = BufReader::new(f);
    let mut graph = HashSet::new();
    let mut i: i32 = 1;

    for line in reader.lines() {
        match line {
            Ok(mut l) => {
                if l.len() < 3 {
                    println!("Line {} does not contain 3 chars => ignored", i);
                    break;
                }
                // Safe, due to test above.
                let e: char = l.pop().unwrap();
                l.pop().unwrap();
                let b: char = l.pop().unwrap();
                graph.insert(Vertice {begin: b, end: e});
            },
            Err(e) => return Err(e),
        };
        i = i+1;
    }

    Ok(graph)
}
