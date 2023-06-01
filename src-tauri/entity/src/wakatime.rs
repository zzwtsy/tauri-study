use sea_orm::{ActiveModelBehavior, Set};
use serde::{Deserialize, Serialize};

use crate::{
    categories, dependencies, editors, grand_total, languages, machines, operating_systems,
    projects, range,
};

#[derive(Debug, Clone, Default)]
pub struct WakaTimeActiveModel {
    pub categories: Vec<categories::ActiveModel>,
    pub dependencies: Vec<dependencies::ActiveModel>,
    pub editors: Vec<editors::ActiveModel>,
    pub grand_total: grand_total::ActiveModel,
    pub languages: Vec<languages::ActiveModel>,
    pub machines: Vec<machines::ActiveModel>,
    pub operating_systems: Vec<operating_systems::ActiveModel>,
    pub projects: Vec<projects::ActiveModel>,
    pub range: range::ActiveModel,
}

pub type WakaTimeModelVec = Vec<WakaTimeModel>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WakaTimeModel {
    pub categories: Vec<categories::Model>,
    pub dependencies: Vec<dependencies::Model>,
    pub editors: Vec<editors::Model>,
    pub grand_total: grand_total::Model,
    pub languages: Vec<languages::Model>,
    pub machines: Vec<machines::Model>,
    pub operating_systems: Vec<operating_systems::Model>,
    pub projects: Vec<projects::Model>,
    pub range: range::Model,
}

impl From<WakaTimeModel> for WakaTimeActiveModel {
    fn from(json: WakaTimeModel) -> Self {
        let id = get_scru128_id();

        WakaTimeActiveModel {
            categories: map_categories(json.categories, id),
            dependencies: map_dependencies(json.dependencies, id),
            editors: map_editors(json.editors, id),
            grand_total: map_grand_total(json.grand_total, id),
            languages: map_languages(json.languages, id),
            machines: map_machines(json.machines, id),
            operating_systems: map_operating_systems(json.operating_systems, id),
            projects: map_projects(json.projects, id),
            range: map_range(json.range, id),
        }
    }
}

fn map_range(range: range::Model, id: i64) -> range::ActiveModel {
    let mut tmp = range::ActiveModel::new();

    tmp.id = Set(id);
    tmp.date = Set(range.date);
    tmp.end = Set(range.end);
    tmp.start = Set(range.start);
    tmp.text = Set(range.text);
    tmp.timezone = Set(range.timezone);

    tmp
}

fn map_grand_total(grand_total: grand_total::Model, range_id: i64) -> grand_total::ActiveModel {
    let mut tmp = grand_total::ActiveModel::new();

    tmp.decimal = Set(grand_total.decimal);
    tmp.hours = Set(grand_total.hours);
    tmp.minutes = Set(grand_total.minutes);
    tmp.text = Set(grand_total.text);
    tmp.total_seconds = Set(grand_total.total_seconds);
    tmp.range_id = Set(range_id);
    tmp.digital = Set(grand_total.digital);

    tmp
}

fn map_categories(
    categories: Vec<categories::Model>,
    range_id: i64,
) -> Vec<categories::ActiveModel> {
    categories
        .into_iter()
        .map(|category| {
            let mut tmp = categories::ActiveModel::new();

            tmp.decimal = Set(category.decimal);
            tmp.digital = Set(category.digital);
            tmp.hours = Set(category.hours);
            tmp.minutes = Set(category.minutes);
            tmp.name = Set(category.name);
            tmp.percent = Set(category.percent);
            tmp.seconds = Set(category.seconds);
            tmp.text = Set(category.text);
            tmp.total_seconds = Set(category.total_seconds);
            tmp.range_id = Set(range_id);

            tmp
        })
        .collect()
}

fn map_dependencies(
    dependencies: Vec<dependencies::Model>,
    range_id: i64,
) -> Vec<dependencies::ActiveModel> {
    dependencies
        .into_iter()
        .map(|dependency| {
            let mut tmp = dependencies::ActiveModel::new();

            tmp.decimal = Set(dependency.decimal);
            tmp.digital = Set(dependency.digital);
            tmp.hours = Set(dependency.hours);
            tmp.minutes = Set(dependency.minutes);
            tmp.name = Set(dependency.name);
            tmp.percent = Set(dependency.percent);
            tmp.seconds = Set(dependency.seconds);
            tmp.text = Set(dependency.text);
            tmp.total_seconds = Set(dependency.total_seconds);
            tmp.range_id = Set(range_id);

            tmp
        })
        .collect()
}

fn map_editors(editors: Vec<editors::Model>, range_id: i64) -> Vec<editors::ActiveModel> {
    editors
        .into_iter()
        .map(|editor| {
            let mut tmp = editors::ActiveModel::new();

            tmp.decimal = Set(editor.decimal);
            tmp.digital = Set(editor.digital);
            tmp.hours = Set(editor.hours);
            tmp.minutes = Set(editor.minutes);
            tmp.name = Set(editor.name);
            tmp.percent = Set(editor.percent);
            tmp.seconds = Set(editor.seconds);
            tmp.text = Set(editor.text);
            tmp.total_seconds = Set(editor.total_seconds);
            tmp.range_id = Set(range_id);

            tmp
        })
        .collect()
}

fn map_languages(languages: Vec<languages::Model>, range_id: i64) -> Vec<languages::ActiveModel> {
    languages
        .into_iter()
        .map(|language| {
            let mut tmp = languages::ActiveModel::new();

            tmp.decimal = Set(language.decimal);
            tmp.digital = Set(language.digital);
            tmp.hours = Set(language.hours);
            tmp.minutes = Set(language.minutes);
            tmp.name = Set(language.name);
            tmp.percent = Set(language.percent);
            tmp.seconds = Set(language.seconds);
            tmp.text = Set(language.text);
            tmp.total_seconds = Set(language.total_seconds);
            tmp.range_id = Set(range_id);

            tmp
        })
        .collect()
}

fn map_machines(machines: Vec<machines::Model>, range_id: i64) -> Vec<machines::ActiveModel> {
    machines
        .into_iter()
        .map(|machine| {
            let mut tmp = machines::ActiveModel::new();

            tmp.decimal = Set(machine.decimal);
            tmp.digital = Set(machine.digital);
            tmp.hours = Set(machine.hours);
            tmp.minutes = Set(machine.minutes);
            tmp.name = Set(machine.name);
            tmp.percent = Set(machine.percent);
            tmp.seconds = Set(machine.seconds);
            tmp.text = Set(machine.text);
            tmp.total_seconds = Set(machine.total_seconds);
            tmp.machine_name_id = Set(machine.machine_name_id);
            tmp.range_id = Set(range_id);

            tmp
        })
        .collect()
}

fn map_operating_systems(
    operating_systems: Vec<operating_systems::Model>,
    range_id: i64,
) -> Vec<operating_systems::ActiveModel> {
    operating_systems
        .into_iter()
        .map(|operating_system| {
            let mut tmp = operating_systems::ActiveModel::new();

            tmp.minutes = Set(operating_system.minutes);
            tmp.seconds = Set(operating_system.seconds);
            tmp.decimal = Set(operating_system.decimal);
            tmp.digital = Set(operating_system.digital);
            tmp.hours = Set(operating_system.hours);
            tmp.name = Set(operating_system.name);
            tmp.percent = Set(operating_system.percent);
            tmp.text = Set(operating_system.text);
            tmp.total_seconds = Set(operating_system.total_seconds);
            tmp.range_id = Set(range_id);

            tmp
        })
        .collect()
}

fn map_projects(projects: Vec<projects::Model>, range_id: i64) -> Vec<projects::ActiveModel> {
    projects
        .into_iter()
        .map(|project| {
            let mut tmp = projects::ActiveModel::new();

            tmp.decimal = Set(project.decimal);
            tmp.digital = Set(project.digital);
            tmp.hours = Set(project.hours);
            tmp.minutes = Set(project.minutes);
            tmp.name = Set(project.name);
            tmp.percent = Set(project.percent);
            tmp.seconds = Set(project.seconds);
            tmp.text = Set(project.text);
            tmp.total_seconds = Set(project.total_seconds);
            tmp.range_id = Set(range_id);

            tmp
        })
        .collect()
}

fn get_scru128_id() -> i64 {
    scru128::new().timestamp().try_into().unwrap()
}
