extern crate rlua;

use rlua::*;

fn main() {
    struct Test {
        field: i32,
    }

    let lua = Lua::new();
    lua.scope(|scope| {
        let mut inner: Option<Table> = None;
        let f = scope
            .create_function_mut(|_, t: Table| {
                //~^ error: cannot infer an appropriate lifetime for autoref due to conflicting requirements
                inner = Some(t);
                Ok(())
            })
            .unwrap();
        f.call::<_, ()>(lua.create_table()).unwrap();
    });
}