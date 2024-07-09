use mlua::{FromLua, Lua};
use mlua_test::ToTable;

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

            // lua.load(r#"print(global.frame_count)"#).exec()?;

            self.frame_count += 1;
        }
    }

    pub fn set_global_table(&self, lua: &Lua) -> Result<(), anyhow::Error> {
        let global_table = lua.create_table()?;

        global_table.set("frame_count", self.frame_count)?;

        lua.globals().set("global", global_table)?;

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let lua = Lua::new();

    let mut application = Application::new();

    application.start(&lua)?;

    Ok(())
}
