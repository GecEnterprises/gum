use ts_rs::TS;

#[derive(TS)]
#[ts(export)]
struct GumDefinition {
    user_id: i32,
    first_name: String,
    last_name: String,
}

