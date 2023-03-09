pub mod implementv1 {

    use figlet_rs::FIGfont;
    pub fn runme1(text: String) -> (usize, usize) {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let mut n_vovels = 0_usize;
        let mut n_total = 0_usize;
        let chars: Vec<char> = text.chars().collect();
        for character in chars {
            if vowels.contains(&character) {
                n_vovels += 1;
            }
            n_total += 1;
        }
        (n_total, n_vovels)
    }
}
