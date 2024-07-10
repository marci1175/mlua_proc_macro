use mlua::Lua;
use mlua_proc_macro::ToTable;

#[derive(ToTable, serde::Serialize, serde::Deserialize)]
struct Application {
    frame_count: i64,

    #[table(skip)]
    test: Info,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct Info {
    name: String,
}

impl Application {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            test: Info::default(),
        }
    }

    pub fn start(&mut self, lua: &Lua) -> anyhow::Result<()> {
        loop {
            self.set_table_from_struct(lua);

            // lua.load(r#"print(vars.test)"#).exec()?;

            self.frame_count += 1;
        }
    }
}

fn main() -> anyhow::Result<()> {
    let lua = Lua::new();

    let mut application = Application::new();

    application.start(&lua)?;

    Ok(())
}
