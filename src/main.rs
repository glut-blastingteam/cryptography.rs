fn encode_caesar(content: &str, cipher:u32) -> String{
    let mut encrypted:String=String::new();

    for i in content.as_bytes(){
        let num = (*i as u32 + cipher) % 26;
        encrypted.push(std::char::from_u32(num).unwrap());
    }
    encrypted
}

fn main() {
    for i in 0..100000{
        assert_eq!(encode_caesar("hello,world",i),encode_caesar("hello,world",i));
    }

}
