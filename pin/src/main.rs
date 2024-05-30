fn main() {
    println!("Hello, world!");
}

/*
┌───┬───┬───┐
│ 1 │ 2 │ 3 │
├───┼───┼───┤
│ 4 │ 5 │ 6 │
├───┼───┼───┤
│ 7 │ 8 │ 9 │
└───┼───┼───┘
    │ 0 │
    └───┘ */

fn get_pins(observed: &str) -> Vec<String> {
    return allpossible(allarr(observed));
}

fn adjency(dig: char) -> Vec<char> {
    let mut vec: Vec<char> = Vec::new();
    if dig.is_digit(10) {
        vec.push(dig);
    }
    vec.push(dig);
    match dig {
        '1' => {
            vec.push('2');
            vec.push('4')
        }
        '2' => {
            vec.push('5');
            vec.push('1');
            vec.push('3')
        }
        '3' => {
            vec.push('2');
            vec.push('6')
        }
        '4' => {
            vec.push('7');
            vec.push('1');
            vec.push('5')
        }
        '5' => {
            vec.push('8');
            vec.push('4');
            vec.push('2');
            vec.push('6')
        }
        '6' => {
            vec.push('9');
            vec.push('3');
            vec.push('5')
        }
        '7' => {
            vec.push('4');
            vec.push('8')
        }
        '8' => {
            vec.push('0');
            vec.push('5');
            vec.push('7');
            vec.push('9')
        }
        '9' => {
            vec.push('6');
            vec.push('8')
        }
        '0' => vec.push('8'),
        _ => return vec,
    }
    return vec;
}

fn allarr(observed: &str) -> Vec<Vec<char>> {
    let mut vec: Vec<Vec<char>> = Vec::new();
    for i in observed.chars() {
        vec.push(adjency(i))
    }

    return vec;
}

fn allpossible(mut vec: Vec<Vec<char>>) -> Vec<String> {
    let mut vec1: Vec<String> = Vec::new();

    let first = vec.remove(0);

    first.iter().for_each(|x| vec1.push(x.to_string()));

    let mut vec1mut: Vec<String> = Vec::new();

    for x in vec {
        for i in x {
            vec1.iter().for_each(|x: &String| {
                let mut y = x.clone();
                y.push(i);
                vec1mut.push(y.to_string());
            });
        }

        vec1 = vec1mut;
        vec1mut = Vec::new();
    }
    vec1.sort();
    vec1.dedup();
    return vec1;
}
