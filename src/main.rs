fn main(){
    for num in 1..1000{
        
        if num % 5 == 0 {
            println!("{}だワン！", num);//5の倍数のとき犬っぽくなる
        } else if num % 3 == 0{
            println!("{}！ｗ", num);//3の倍数のときアホっぽくなる
        }else if num.to_string().contains('3'){
            println!("{}！ｗ", num);//3がつくときアホっぽくなる
        }else {
            println!("{}", num);//普通に数える
        }
    }
}