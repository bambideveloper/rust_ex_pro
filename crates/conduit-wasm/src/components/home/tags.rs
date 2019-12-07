use stdweb::web::event::IEvent;
use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::agent::Tags as TagsAgent;
use crate::error::Error;
use crate::types::TagListInfo;

pub struct Tags {
    tags: TagsAgent,
    tag_list: Option<TagListInfo>,
    response: Callback<Result<TagListInfo, Error>>,
    task: Option<FetchTask>,
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub callback: Callback<String>,
}

pub enum Msg {
    Response(Result<TagListInfo, Error>),
    TagFiltered(String),
}

impl Component for Tags {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        Tags {
            tags: TagsAgent::new(),
            tag_list: None,
            response: link.send_back(Msg::Response),
            task: None,
            props,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        let task = self.tags.get_all(self.response.clone());
        self.task = Some(task);
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Response(Ok(tag_list)) => {
                self.tag_list = Some(tag_list);
            }
            Msg::Response(Err(_)) => {}
            Msg::TagFiltered(tag) => {
                self.props.callback.emit(tag);
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        if let Some(tag_list) = &self.tag_list {
            html! {
                <div className="tag-list">
                    {for tag_list.tags.iter().map(|tag| {
                        let tag_filtered = tag.clone();
                        html! {
                            <a
                                href=""
                                class="tag-default tag-pill"
                                onclick=|ev| { ev.prevent_default(); Msg::TagFiltered(tag_filtered.clone()) }>
                                { &tag }
                            </a>
                        }
                    })}
                </div>
            }
        } else {
            html! {
                <div>{ "Loading Tags..." }</div>
            }
        }
    }
}
