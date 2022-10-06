pub struct CDStartup {
    opening_message: String,
    title: String,
    logo: String,
}

impl CDStartup {
    pub fn new(t: String, o: String) -> CDStartup {
        CDStartup {
            logo: CDStartup::read_logo(),
            title: t,
            opening_message: o,
        }
    }

    fn read_logo() -> String {
        String::from(
            "
                     ＿＿
        　　　　　🌸＞　　フ
        　　　　　| 　_　 _ l
        　 　　　／` ミ＿xノ
        　　 　 /　　　 　 |
        　　　 /　 ヽ　　 ﾉ
        　 　 │　　|　|　|
        　／￣|　　 |　|　|
        　| (￣ヽ＿_ヽ_)__)
        　＼二つ
        ",
        )
    }

    pub fn print_opening_message(self) {
        println!("\x1b[38;5;205m{}", self.logo);
        println!("{}", self.title);
        println!("{}\x1b[0m", self.opening_message);
    }
}

impl Default for CDStartup {
    fn default() -> Self {
        CDStartup::new(String::from("Def"), String::from("test"))
    }
}
