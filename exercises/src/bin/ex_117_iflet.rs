
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    let a = Foo::Qux(10);

    // Remove the codes below, using `match` instead 
    if let Foo::Bar = a {
        println!("match foo::bar")
    } else if let Foo::Baz = a {
        println!("match foo::baz")
    } else {
        println!("match others")
    }
    match a {
        Foo::Bar => println!("Bar"),
        Foo::Qux(i)=>println!("qux:{i}"),
        Foo::Baz => println!("Baz"),
    };
}