extern crate curl;
extern crate scraper;
extern crate std;

pub struct Character {
    pub name: String,
    pub account_name: String,
    pub class: String,
    pub level: String,
    pub server: String,
    pub faction: String,
    pub faction_rank: String,
    pub clan: String
}

impl Character {
    pub fn new(domain: String, region: String, name: String) -> Result<Character, String> {

        if region != "eu" && region != "na" {
            return Err("INVALID_REGION".to_string())
        }

        let mut raw_data = Vec::new();
        {
            let mut easy = curl::easy::Easy::new();

            let escaped_name = easy.url_encode(name.as_bytes());

            let url = format!("http://{}-bns.{}/ingame/bs/character/profile?c={}",
                              region, domain, escaped_name);
            match easy.url(&url) {
                Ok(_) => {},
                Err(_) => return Err("Bad URL.".to_string())
            }
            let mut transfer = easy.transfer();
            let res = transfer.write_function(|request_data| {
                raw_data.extend_from_slice(request_data);
                Ok(request_data.len())
            });
            match res {
                Ok(_) => {},
                Err(_) => return Err("Bad trasfer callback.".to_string())
            }
            match transfer.perform() {
                Ok(_) => {},
                Err(_) => return Err("Error while fetching data.".to_string())
            }
        }
        let data = match String::from_utf8(raw_data) {
            Ok(data) => data,
            Err(_) => return Err("Data website is broken.".to_string())
        };


        let document = scraper::Html::parse_document(&data);
        let sel_acc = match scraper::Selector::parse("dl.signature dt a") {
            Ok(x) => x,
            Err(_) => return Err("Error while getting account data.".to_string())
        };

        let sel_char = match scraper::Selector::parse("dd.desc ul li")  {
            Ok(x) => x,
            Err(_) => return Err("Error while getting character data.".to_string())
        };

        let acc_name = match document.select(&sel_acc).next() {
                Some(x) => match x.text().next() {
                    Some(t) => String::from(t),
                    None => return Err("CHARACTER_NOT_FOUND".to_string())
                },
                None => return Err("CHARACTER_NOT_FOUND".to_string())
        };


        let rest_sig = document.select(&sel_char).collect::<Vec<_>>();

        let mut rest = Vec::new();

        for ch in rest_sig  {
            match ch.first_child() {
                Some(f_ch) => {
                    match f_ch.value().as_text() {
                        Some(t) => {
                            rest.push(match std::str::from_utf8(t.as_bytes()){
                                Ok(x) => x,
                                Err(_) => ""
                            }.to_owned());
                        },
                        None => {}
                }
                },
                None => {}
            }
        };

        if rest.len() < 3 {
            return Err("Error while parsing data.".to_string());
        };

        let class = rest[0].clone();
        let level = rest[1].clone();
        let server = rest[2].clone();

        let mut faction = String::new();
        let mut faction_rank = String::new();
        let mut clan = String::new();

        if rest.len() >= 4 {
            faction = rest[3].split_whitespace().take(2).fold(String::new(), str_joiner).trim().to_owned();
            faction_rank = rest[3].split_whitespace().skip(2).fold(String::new(), str_joiner).trim().to_owned();
        }

        if rest.len() >= 5 {
            clan = rest[4].clone();
        }

        let chara = Character {
            name: name,
            account_name: acc_name,
            class: class,
            level: level,
            server: server,
            faction: faction,
            faction_rank: faction_rank,
            clan: clan
        };

        Ok(chara)
    }
}

impl std::fmt::Debug for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{} ({})\nClass: {}\nLevel: {}\nServer: {}\nFaction: {} ({})\nClan: {}\n",
            self.name, self.account_name, self.class, self.level, self.server, self.faction, self.faction_rank, self.clan
        )
    }
}

fn str_joiner(acc: String, x: &str) -> String {
    acc + " " + x
}
