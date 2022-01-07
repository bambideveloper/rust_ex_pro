use wasm_bindgen_futures::spawn_local;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;
use crate::services::articles::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub slug: String,
    pub can_modify: bool,
}

/// Article actions component to edit or delete an article.
#[function_component(ArticleActions)]
pub fn article_actions(props: &Props) -> Html {
    let history = use_history().unwrap();
    let onclick = {
        let slug = props.slug.clone();
        Callback::from(move |_| {
            let slug = slug.clone();
            let history = history.clone();
            spawn_local(async move {
                if del(slug).await.is_ok() {
                    history.push(AppRoute::Home);
                }
            });
        })
    };

    if props.can_modify {
        html! {
            <span>
                <Link<AppRoute> to={AppRoute::Editor { slug: props.slug.clone() }} classes="btn btn-outline-secondary btn-sm" >
                    { "Edit Article" }
                </Link<AppRoute>>
                { " " }
                <button class="btn btn-outline-danger btn-sm" {onclick} >
                    <i class="ion-trash-a"></i> { "Delete Article" }
                </button>
            </span>
        }
    } else {
        html! {
            <span>
            </span>
        }
    }
}
