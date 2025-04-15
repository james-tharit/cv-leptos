use leptos::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CurriculumVitae {
    short_summary: String,
    technical_skills: Vec<String>,
    experiences: Vec<Experience>,
}

#[derive(Debug, Deserialize, Clone)]
struct Experience {
    title: String,
    period: String,
    location: String,
    key_achievements: Vec<String>,
    technical_skills: String,
}

fn get_cv() -> CurriculumVitae {
    // note: read file like this will cause panic `Uncaught RuntimeError: unreachable`
    // That works on your local machine, but when compiled to WebAssembly and run in the browser,
    // WASM has no access to a local filesystem — thus calling fs::File::open in the browser will instantly panic.
    // let mut file = fs::File::open("data.json").expect("Unable to open the file");
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)
    //     .expect("Unable to read the file");

    let contents = include_str!("../data.json");

    let cv: CurriculumVitae = serde_json::from_str(&contents).expect("JSON was not well-formatted");
    cv
}

fn main() {
    let cv = get_cv();
    leptos::mount::mount_to_body(|| App(AppProps { cv: (cv) }))
}

#[component]
fn Headers(short_summary: String) -> impl IntoView {
    view! {
        <main class="my-0 mx-auto max-w-3xl text-center">
            <h1>James Tharit Rattananenya</h1>
            <p class="p-6 text-2xl">{short_summary}</p>
        </main>
    }
}

#[component]
fn Achievements(achievements: Vec<String>) -> impl IntoView {
    view! {
        {
            achievements.into_iter().map(|ach| {
                view! {
                    <li>{ach}</li>
                }
            }
        ).collect_view()}
    }
}

#[component]
fn Experiences(experiences: Vec<Experience>) -> impl IntoView {
    let experience_section = experiences
        .into_iter()
        .map(|exp| {
            view! {
                <h3>{ exp.title }</h3>
                <p class="italic">{ exp.location }</p>
                <p class="italic">{ exp.period }</p>
                <Achievements achievements=exp.key_achievements />
                <p class="bold">{ exp.technical_skills }</p>
            }
        })
        .collect_view();

    view! {
        <h2>Experiences</h2>
        {experience_section}
    }
}

#[component]
fn Skills(skills: Vec<String>) -> impl IntoView {
    let skills = skills
        .into_iter()
        .map(|skill| {
            view! {
               <li>{skill}</li>
            }
        })
        .collect_view();

    view! {
        <h2>Technical Skills</h2>
        {skills}
    }
}

#[component]
fn App(cv: CurriculumVitae) -> impl IntoView {
    view! {
        <div class="p-3 mt-4 mb-4 w-1/2 mx-auto">
            <Headers short_summary=cv.short_summary />
            <Skills skills=cv.technical_skills />
            <Experiences experiences=cv.experiences />
        </div>

    }
}
