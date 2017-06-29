#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexAttributes {
    name: String,
    created_at: String,
    updated_at: String,
    entries: u32,
    data_size: u32,
    file_size: u32,
    last_build_time_s: u32,
    pending_task: bool,
    number_of_pending_tasks: u32,
}

#[derive(Deserialize)]
pub struct Indices {
    items: Vec<IndexAttributes>,
}
