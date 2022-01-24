use std::collections::HashMap;

use crate::errors::FetchError;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{Event, HtmlInputElement, Request, RequestInit, RequestMode, Response};
use yew::{html, html::TargetCast, Component, Context, Html};

use gloo_file::{callbacks::FileReader, File};
use serde::Deserialize;

const OMAHA_URL: &str = "https://block-tools.damopan.com.cn";

pub enum Msg {
    Files(Vec<File>),
    Update(String, String),
    Loads(UpdateResult),
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateResult {
    #[serde(skip_deserializing)]
    filename: String,
    result: String,
    note: String,
}

pub struct Model {
    readers: HashMap<String, FileReader>,
    files: Vec<UpdateResult>,
}

async fn fetch_upload_str(data: String) -> Result<UpdateResult, FetchError> {
    // 如果是Vec<u8> 的话，需要先序列化成对象
    // 然后在转换成json字符串
    //
    // let info: Info = serde_json::from_slice(&bytes).unwrap();
    // let json_str = serde_json::to_string(&info).unwrap();
    // opts.body(Some(&JsValue::from_str(&json_str)));
    //
    let mut opts = RequestInit::new();
    opts.method("Post");

    // 可以不写，默认为 Cors
    opts.mode(RequestMode::Cors);
    opts.body(Some(&JsValue::from_str(&data)));

    let url = OMAHA_URL.to_string() + "/add_payload";
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.json()?).await?;
    log::info!("json {:?}", json);
    let update_result: UpdateResult = json.into_serde().unwrap();
    log::info!("update result {:?}", update_result);
    Ok(update_result)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            files: vec![],
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div>
                    <p>{ "Choose a file to upload to see the uploaded bytes" }</p>
                    <input type="file" multiple=true onchange={ctx.link().callback(move |e: Event| {
                            let mut result = Vec::new();
                            let input: HtmlInputElement = e.target_unchecked_into();

                            if let Some(files) = input.files() {
                                let files = js_sys::try_iter(&files)
                                    .unwrap()
                                    .unwrap()
                                    .map(|v| web_sys::File::from(v.unwrap()))
                                    .map(File::from);
                                result.extend(files);
                            }
                            Msg::Files(result)
                        })}
                    />
                </div>
                <div>
                </div>
                <table>
                  <tr>
                    <th> { "filename" } </th>
                    <th> { "result" } </th>
                    <th> { "note" } </th>
                  </tr>
                    { for self.files.iter().map(|f| Self::view_file(f)) }
                </table>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Files(files) => {
                for file in files.into_iter() {
                    let file_name = file.name();
                    let link = ctx.link().clone();

                    // callbacks需要被调用才能执行
                    // 所以加一个readers存储
                    let task = {
                        let file_name = file_name.clone();
                        gloo_file::callbacks::read_as_text(&file, move |res| {
                            let bytes = res.expect("faile to upload file");
                            link.send_message(Msg::Update(file_name, bytes))
                        })
                    };

                    self.readers.insert(file_name, task);
                }
                true
            }

            Msg::Update(file_name, data_str) => {
                let link = ctx.link().clone();
                let file_name_inner = file_name.clone();

                spawn_local(async move {
                    let mut update_result = fetch_upload_str(data_str).await.unwrap();
                    update_result.filename = file_name_inner;
                    link.send_message(Msg::Loads(update_result));
                });
                self.readers.remove(&file_name);
                true
            }

            Msg::Loads(update_result) => {
                self.files.push(update_result);
                true
            }
        }
    }
}

impl Model {
    fn view_file(update_result: &UpdateResult) -> Html {
        html! {
             <tr>
               <td>{ &update_result.filename }</td>
               <td>{ &update_result.result }</td>
               <td>{ &update_result.note }</td>
             </tr>
        }
    }
}
