use data::modules::get_module_list;
use responses::{APIResponse, ok};

#[get("/")]
pub fn get_info() -> APIResponse {

    let module_list = get_module_list();
    ok().message("Research data.").data(json!(&module_list))
}