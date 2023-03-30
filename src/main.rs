fn median(mut a: Vec<f32>) -> Option<f32> {

    let len: usize = a.len();

    if len == 0 {
        return None;
    }

    a.sort_by(|a,b| a.partial_cmp(b).unwrap());

    let mid: f32;

    if len % 2 == 1 {
        mid = a[(len / 2) as usize];
    } else {
        mid = (a[(len / 2) as usize] + a[(len / 2 - 1) as usize]) / 2.0;
    }

    return Some(mid);
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
