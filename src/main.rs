use mlua::Lua;
use mlua_proc_macro::ToTable;

#[derive(ToTable, serde::Serialize, serde::Deserialize, Clone)]
pub struct Application {
    pub frame_count: i64,

    #[table(save)]
    pub info: Info,

    pub field1: String,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Info {
    pub name: String,
}

impl Application {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            info: Info::default(),
            field1: String::from("Hello world!"),
        }
    }

    pub fn start(&mut self, lua: &Lua) -> anyhow::Result<()> {
        loop {
            self.clone().set_lua_table_function(lua);

            lua.load(r#"print(application.field1)"#).exec()?;

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
