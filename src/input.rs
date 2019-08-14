use cachedir::{CacheDir, CacheDirConfig};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Result, Write},
};

pub struct GetInput {
    year: String,
    directory: CacheDir,
}

impl GetInput {
    pub fn new<Y>(year: Y) -> Result<Self>
    where
        Y: Into<String>,
    {
        let year = year.into();

        let path = format!("AdventOfCode/{}", year);
        let cache = CacheDirConfig::new(&path).get_cache_dir();

        Ok(Self {
            year: year.into(),
            directory: cache?,
        })
    }

    pub fn get_input(&self, day: u8) -> Result<String> {
        if let Ok(input) = self.get_cached(day) {
            return Ok(input);
        }

        let input = self.fetch_input(day)?;
        self.cache_input(day, &input)?;

        Ok(input)
    }

    fn get_cached(&self, day: u8) -> Result<String> {
        let location = self.directory.join(format!("day{}", day));
        let f = File::open(location)?;

        let mut reader = BufReader::new(f);

        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;

        Ok(buffer)
    }

    fn fetch_input(&self, day: u8) -> Result<String> {
        let cookie = self.read_cookie()?;
        let cookie = format!("session={}", cookie);

        let input = ureq::get(&format!(
            "https://adventofcode.com/{}/day/{}/input",
            self.year, day
        ))
        .set("Cookie", &cookie)
        .call()
        .into_string()?;

        Ok(input)
    }

    fn cache_input<B>(&self, day: u8, s: B) -> Result<()>
    where
        B: AsRef<[u8]>,
    {
        let location = self.directory.join(format!("day{}", day));
        let f = File::create(location)?;

        let mut writer = BufWriter::new(f);

        writer.write_all(s.as_ref())?;

        Ok(())
    }

    pub fn write_cookie(cookie: &str, year: String) -> Result<()> {
        let inputs = Self::new::<String>(year)?;

        let location = inputs.directory.join("cookie");
        let f = File::create(location)?;

        let mut writer = BufWriter::new(f);

        writer.write_all(cookie.as_bytes())?;

        Ok(())
    }

    fn read_cookie(&self) -> Result<String> {
        let location = self.directory.join("cookie");
        let f = File::open(location)?;

        let mut reader = BufReader::new(f);
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;

        Ok(buffer)
    }
}
