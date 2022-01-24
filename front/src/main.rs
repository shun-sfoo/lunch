use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod error;
mod hooks;
mod pages;
mod routes;
mod services;
mod types;
use pages::{home, payload, upload};

use yew::html::Scope;

/// App routes
#[derive(Routable, PartialEq, Clone, Debug)]
pub enum AppRoute {
    #[at("/payload_list")]
    PayloadList,
    #[at("/payload")]
    AddPayLoad,
    #[at("/")]
    Home,
}

pub enum Msg {
    ToggleNavBar,
}

pub struct Model {
    navbar_active: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavBar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }

                <main>
                    <Switch<AppRoute> render={Switch::render(switch)} />
                </main>
            </BrowserRouter>
        }
    }
}

impl Model {
    fn view_nav(&self, _link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;
        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class={classes!("navbar-menu", active_class)}>
                     <div class="navbar-start">
                         <Link<AppRoute> classes={classes!("navbar-item")} to={AppRoute::Home}>
                            { " Home " }
                         </Link<AppRoute>>
                         <Link<AppRoute> classes={classes!("navbar-item")} to={AppRoute::PayloadList}>
                            { " List " }
                         </Link<AppRoute>>
                         <Link<AppRoute> classes={classes!("navbar-item")} to={AppRoute::AddPayLoad}>
                            { " Upload " }
                         </Link<AppRoute>>
                     </div>
                </div>
            </nav>
        }
    }
}

fn switch(routes: &AppRoute) -> Html {
    match routes.clone() {
        AppRoute::PayloadList => {
            html! { <payload::Model/> }
        }
        AppRoute::AddPayLoad => {
            html! { <upload::Model />}
        }
        AppRoute::Home => {
            html! { <home::Model /> }
        }
    }
}

pub fn main() {
    yew::start_app::<Model>();
}
