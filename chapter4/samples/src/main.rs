fn main() {
    /*******************************************************************************************/
    /*    Rust               |    C/C++                |   Description                         */
    /*******************************************************************************************/
    /*    a: &T              |    const a *const T     |   can't mutate either                 */
    /*    mut a: &T          |    const a *T           |   can't mutate what is pointed to     */
    /*    a: &mut T          |    a *const T           |   can't mutate pointer                */
    /*    mut a: &mut T      |    a *T                 |   can mutate both                     */
    /*******************************************************************************************/

    let val1: &mut u8 = &mut 2u8;
    *val1 = 5u8;
    println!("val1 is: {val1}");
    // val1 = &mut 7u8;

    let mut val2: &u8 = &9u8;
    // *val2=10u8;
}