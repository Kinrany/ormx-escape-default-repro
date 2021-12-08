#[derive(Clone, Debug, ormx::Table)]
#[ormx(table = "NewTable", id = id, insertable)]
struct NewTable {
    #[ormx(default)]
    id: i32,
    somefield: String,
}
