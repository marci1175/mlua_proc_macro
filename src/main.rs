use mlua::{FromLua, Lua, Table};
use mlua_proc_macro::ToTable;

#[derive(ToTable)]
struct Application {
    frame_count: i64,
}

impl Application {
    pub fn new() -> Self {
        Self { frame_count: 0 }
    }

    pub fn start(&mut self, lua: &Lua) -> anyhow::Result<()> {
        loop {
            self.set_global_table(lua)?;

            lua.load(r#"print(vars.frame_count)"#).exec()?;

            self.frame_count += 1;
        }
    }

    pub fn set_global_table(&self, lua: &Lua) -> Result<(), anyhow::Error> {
        self.set_table_from_struct(&lua);

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let lua = Lua::new();

    let mut application = Application::new();

    application.start(&lua)?;

    Ok(())
}
