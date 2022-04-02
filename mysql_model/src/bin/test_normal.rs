pub fn main() {
    let v1 = vec![1,2,3];
    let v2 = vec!["a","b","c","d"];

    for i in  v1.iter().zip(v2.iter()) {
        print!("{:?}",i);
    }

}