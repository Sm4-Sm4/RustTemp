

pub fn bits(tableau:Vec<usize>) -> Vec<String>{
    let mut bvec = Vec::new();
    for element in tableau{
        let ebinairy = format!("{:b}",element);
        bvec.push(ebinairy);
    }
    return  bvec;
}


fn main(){

let tableau = vec![23,213,51,123,1232];
let var =  bits(tableau);
println!("{var:?}")

}