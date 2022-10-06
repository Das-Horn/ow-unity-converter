pub struct CDStartup {
    openingMessage: String,
    title: String,
    logo: String,
}

impl CDStartup {
    pub fn new(t: String, o: String) -> CDStartup {
        CDStartup {
            logo: CDStartup::readLogo(),
            title: t,
            openingMessage: o,
        }
    }

    fn readLogo() -> String {
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

    pub fn printOpeningMessage(self) {
        println!("\x1b[38;5;205m{}", self.logo);
        println!("{}", self.title);
        println!("{}\x1b[0m", self.openingMessage);
    }
}

impl Default for CDStartup {
    fn default() -> Self {
        CDStartup::new(String::from("Def"), String::from("test"))
    }
}
