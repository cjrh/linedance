use linedance::input;

fn main() -> std::io::Result<()> {
    for line in input()? {
        println!("{}", line?);
    }

    Ok(())
}

