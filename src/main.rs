//fn main() -> Result<(), Box<dyn std::error::Error>> {
//}

fn main() {
    println!("start");
    let _ = call();
    println!("end");
}


fn call() -> Result<(), Box<dyn std::error::Error>> {
    let mut rt = tokio::runtime::Runtime::new()?;

    // taskは非同期タスク
    let task = async {
        println!("start");
        tokio::time::delay_for(std::time::Duration::from_secs(1)).await;
        println!("hoge");
        Ok(())
    };
    println!("out of block");
    // プログラムが実行されてから1秒後に"hoge"が表示される
    rt.block_on(task)
}