fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    let number = 3; //shadowing（变量遮蔽），超过scope会变回 shadow前的值
    println!("Number plus two is: {}", number + 2);
}
