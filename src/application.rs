use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    fn invert(&self) -> Self {
        use Theme::*;
        match self {
            Light => Dark,
            Dark => Light,
        }
    }
}

pub enum Msg {
    SetTheme(Theme),
}

pub struct PeterGow {}

impl Component for PeterGow {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        use Msg::*;
        match msg {
            SetTheme(theme) => {
                LocalStorage::set("theme", theme).expect("Could not set theme")
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let theme = LocalStorage::get("theme").unwrap_or(Theme::Dark);
        set_theme(&theme);
        let invert_colors = {
            let theme = theme.clone();
            ctx.link().callback(move |_| Msg::SetTheme(theme.invert()))
        };
        html! {
            <>
                <header></header>
                <div class="profile-image">
                    <div><div></div></div>
                </div>
                <nav>
                    <h3>{"Peter Gow"}</h3>
                    <a href={"https://github.com/Ninjaman10p"}>{"Github"}</a>
                    <a>{"LinkedIn"}</a>
                    <a>{"Email"}</a>
                    <small>
                        <a href={"https://github.com/Ninjaman10p/portfolio.paelias.xyz"}>{"This website"}</a>
                        {" was written in "}
                        <a href={"https://yew.rs"}>{"Yew"}</a>
                        {", a Rust framework"}
                    </small>
                </nav>
                <main>
                    <h1>{"About Me"}</h1>
                    {"I started programming in 2016 in Grade 7 of High School,
                    learning JavaScript (I intended to learn Java at the time).
                    I've been refining my skills and learning new programming languages
                    and mathematical concepts since then.
                    "}
                    <h1>{"Featured Projects"}</h1>
                    <ul>
                        <li>
                            {"In 2021, I created a PHP-based website for the MacGregor SHS student leadership
                            team, to track and display house points."}
                        </li>
                        <li>
                            {"In 2022, I created "}
                            <a href={"https://github.com/Ninjaman10p/h-mapgen/"}>{"h-mapgen"}</a>
                            {", a randomised map generator using graphs and chunking for optimisation
                            for a contact."}
                        </li>
                        <li>
                            {"In 2022, I created "}
                            <a href={"https://github.com/Ninjaman10p/brainless"}>{"brainless"}</a>
                            {", a transpiler from a python-like language
                            to brainf!%# written in Haskell, in 3 days for the UQCS Hackathon."}
                        </li>
                    </ul>
                    <h1>{"Employment and Education History"}</h1>
                    <ul>
                        <li>{"Ongoing UQ Bachelor of Maths/Science (2021-current)"}</li>
                        <li>{"Casual Tutor at LEE Excellence Education (2021-current)"}</li>
                        <li>{"Casual Salesforce Developer for Coherent Scientific (late 2021-current)"}</li>
                    </ul>
                    <h1>{"Awards and Recognition"}</h1>
                    <ul>
                        <li>{"Dux of MacGregor State High School (2021)"}</li>
                        <li>{"Deans Commendation for Academic Excellence at UQ (2022, both semesters)"}</li>
                        <li>{"Most Masochistic at the 2022 UQCS Hackathon"}</li>
                    </ul>
                    <h1>{"Other Projects"}</h1>
                    <ul>
                        <li>
                            {"A series of small TUI apps and games written using Brick: "}
                            <a href="https://github.com/Ninjaman10p/hrain">{"hrain"}</a>{", "}
                            <a href="https://github.com/Ninjaman10p/snake-hs">{"snake-hs"}</a>{", and "}
                            <a href="https://github.com/Ninjaman10p/checkers-hs">{"checkers-hs"}</a>
                            {" (2022)"}
                        </li>
                        <li>
                            {"In preparation for the UQCS Hackathon 2022, I created "}
                            <a href={"https://github.com/Ninjaman10p/mindmap-hs"}>{"mindmap-hs"}</a>
                            {" to help me organise my notes. mindmap-hs is a
                            TUI mindmapping program written in Haskell using "}
                            <a href={"https://hackage.haskell.org/package/brick"}>{"Brick"}</a>
                            {"."}
                        </li>
                    </ul>
                </main>
                <button class={"theme-toggle"} onclick={invert_colors}><div></div></button>
            </>
        }
    }
}

fn set_theme(theme: &Theme) -> Option<()> {
    let data_theme = match theme {
        Theme::Light => "light",
        Theme::Dark => "dark",
    }
    .to_string();
    web_sys::window()?
        .document()?
        .document_element()?
        .set_attribute("data-theme", &data_theme).ok()?;
    Some(())
}
