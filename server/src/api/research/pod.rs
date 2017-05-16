use diesel::prelude::*;

use helpers::db::DB;

use models::pod::Pod;
use models::user::User;
use models::research::Research;

use schema::pods::dsl as pod_dsl;
use schema::researches::dsl as research_dsl;

use data::types::*;
use data::researches::get_research_list;

use responses::{APIResponse, bad_request, ok};

/// The user needs to be logged in to access this route!
///
/// This route returns the list of all researches and their levels/costs,
/// as well as the current level of the research for the pod of the current user.
#[get("/pod")]
pub fn pod_research(current_user: User, db: DB) -> APIResponse {

    let mut research_list = get_research_list();
    // Create changed pod model and push it to the DB

    let pod = pod_dsl::pods
        .filter(pod_dsl::user_id.eq(current_user.id))
        .get_result::<Pod>(&*db)
        .expect("Failed to get user pod.");

    let pod_result = research_dsl::researches
        .filter(research_dsl::pod_id.eq(pod.id))
        .get_results::<Research>(&*db);

    if pod_result.is_ok() {
        let researches = pod_result.unwrap();
        for research in researches {
            let type_result = ResearchTypes::from_string(&research.name);
            if type_result.is_err() {
                return bad_request()
                           .message(format!("Found research {}, but no matching ResearchType!",
                                            research.name)
                                            .as_str());
            }
            let research_type = type_result.unwrap();
            let list_result = research_list.get_mut(&research_type);
            if list_result.is_none() {
                return bad_request().message(format!("Found type {}, but no matching entry in our research list!", research_type).as_str());
            }

            let mut list_entry = list_result.unwrap();
            list_entry.current_level = Some(research.level);
        }
    }

    ok().message("Research data.").data(json!(&research_list))
}
