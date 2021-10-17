#[cfg(test)]
mod tests {
    #[test]
    fn play_bitwise() {
        let x: u32 = 5;
        let y: u32 = 10;
        let shift: usize =  1;

        println!(
            "{} : {:b}", x, x
        );
        println!(
            "{} : {:b}", y, y
        );
        println!(
            "{}>>{} = {} : {:b}", x, y, x>>y, x>>y
        );
        println!(
            "{}>>{} = {} : {:b}", x, shift, x>>shift, x>>shift
        );
        println!(
            "{}<<{} = {} : {:b}", x, y, x<<y, x<<y
        );
        println!(
            "{}<<{} = {} : {:b}", x, shift, x<<shift, x<<shift
        );
    }
}
