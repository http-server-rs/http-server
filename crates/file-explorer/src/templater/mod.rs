use anyhow::Result;
use chrono::{DateTime, Local};
use handlebars::{handlebars_helper, Handlebars};
use humansize::{format_size, DECIMAL};

use crate::{DirectoryIndex, Sort};

const EXPLORER_KEY: &str = "Explorer";
const EXPLORER_TPL: &[u8] = include_bytes!("./hbs/explorer.hbs");

pub struct Templater {
    pub backend: Handlebars<'static>,
}

impl Templater {
    pub fn new() -> Result<Self> {
        let mut hbs = Handlebars::new();

        let explorer_tpl = String::from_utf8_lossy(EXPLORER_TPL);

        hbs.register_template_string(EXPLORER_KEY, explorer_tpl)?;

        handlebars_helper!(date: |d: Option<DateTime<Local>>| {
            match d {
                Some(d) => d.format("%Y/%m/%d %H:%M:%S").to_string(),
                None => "-".to_owned(),
            }
        });
        hbs.register_helper("date", Box::new(date));

        handlebars_helper!(size: |bytes: u64| format_size(bytes, DECIMAL));
        hbs.register_helper("size", Box::new(size));

        handlebars_helper!(sort_name: |sort: Sort| sort == Sort::Name);
        hbs.register_helper("sort_name", Box::new(sort_name));

        handlebars_helper!(sort_size: |sort: Sort| sort == Sort::Size);
        hbs.register_helper("sort_size", Box::new(sort_size));

        handlebars_helper!(sort_date_created: |sort: Sort| sort == Sort::DateCreated);
        hbs.register_helper("sort_date_created", Box::new(sort_date_created));

        handlebars_helper!(sort_date_modified: |sort: Sort| sort == Sort::DateModified);
        hbs.register_helper("sort_date_modified", Box::new(sort_date_modified));

        Ok(Self { backend: hbs })
    }

    pub fn render(&self, di: &DirectoryIndex) -> Result<String> {
        let tpl = self.backend.render(EXPLORER_KEY, &di)?;
        Ok(tpl)
    }
}
