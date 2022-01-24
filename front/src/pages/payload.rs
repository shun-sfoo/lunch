use chrono::NaiveDateTime;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use yew::prelude::*;

use web_sys::{Request, RequestInit, RequestMode, Response};

use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;

use crate::error::FetchError;

const OMAHA_URL: &str = "https://block-tools.damopan.com.cn";

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Info {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub appid: String,
    pub name: String,
    pub is_delta: bool,
    pub metadata_signature: Option<String>,
    pub metadata_size: i64,
    pub sha256_hex: String,
    pub size: i64,
    pub target_version: String,
    pub version: i32,
    pub published: bool,
    pub publish_at: NaiveDateTime,
}

pub enum Msg {
    UpdateList(Vec<Info>),
}

pub struct Model {
    info_list: Vec<Info>,
}

async fn fetch_info() -> Result<Vec<Info>, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");

    // 可以不写，默认为 Cors
    opts.mode(RequestMode::Cors);

    let url = OMAHA_URL.to_string() + "/payload_list";
    let request = Request::new_with_str_and_init(&url, &opts)?;

    // 构建 yew 的window 对象后，通过 window 对象的 Fetch API，对请求进行分发。
    // 返回的结果类型为 JsValue，通过动态的强制转换方法 dyn_into 将其转换为 web-sys 的 Reponse 类型。
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;
    let info_list: Vec<Info> = json.into_serde().unwrap();
    Ok(info_list)
}

impl Component for Model {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { info_list: vec![] }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let info_list = self.info_list.iter().map(|info| {
            html! {
                  <tr>
                    <td>{ &info.name }</td>
                    <td>{ &info.appid }</td>
                    <td>{ &info.target_version }</td>
                    <td>{ &info.metadata_size }</td>
                    <td>{ &info.size }</td>
                    <td>{ &info.sha256_hex }</td>
                    <td>{ &info.is_delta }</td>
                    <td>{ &info.published }</td>
                    <td>{ &info.publish_at }</td>
                  </tr>
            }
        });

        html! {
            <>
                <p> { "paylod info list" } </p>
                <table>
                  <tr>
                    <th> { "name" } </th>
                    <th> { "appid" } </th>
                    <th> { "target_version" } </th>
                    <th> { "metadata_size" } </th>
                    <th> { "size" } </th>
                    <th> { "sha256_hex" } </th>
                    <th> { "is_delta" } </th>
                    <th> { "published" } </th>
                    <th> { "publish_at" } </th>
                  </tr>
                  { for info_list }
                </table>
            </>

        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateList(res) => {
                self.info_list = res;
                true
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let link = ctx.link().clone();
        if first_render {
            spawn_local(async move {
                let res = fetch_info().await;
                link.send_message(Msg::UpdateList(res.unwrap()))
            });
        }
    }
}
