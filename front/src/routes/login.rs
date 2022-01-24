use wasm_bindgen_futures::spawn_local;
use web_sys::{FocusEvent, HtmlInputElement, InputEvent};

use crate::components::list_errors::ListErrors;
use crate::services::auth::*;
use yew::prelude::*;

use crate::{
    hooks::use_user_context::use_user_content,
    types::{LoginInfo, LoginInfoWrapper},
};

#[function_component(Login)]
pub fn login() -> Html {
    let user_ctx = use_user_content();
    let error = use_state(|| None);
    let login_info = use_state(LoginInfo::default);

    let onsubmit = {
        let error = error.clone();
        let login_info = login_info.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            let request = LoginInfoWrapper {
                user: (*login_info).clone(),
            };

            let user_ctx = user_ctx.clone();
            spawn_local(async move {
                let user_info = login(request).await;
                match user_info {
                    Ok(user_info) => {
                        user_ctx.login(user_info.user);
                        error.set(None);
                    }

                    Err(e) => error.set(Some(e)),
                }
            });
        })
    };

    let oninput_password = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.password = input.value();
            login_info.set(info);
        })
    };

    html! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">{ "登录" }</h1>
                        <ListErrors error={(*error).clone()} />
                        <form {onsubmit}>
                            <fieldset>
                                <fieldset class="form-grounp">
                                    <input
                                        class="form-control form-control-lg"
                                        type="password"
                                        placeholder="Password"
                                        value={login_info.password.clone()}
                                        oninput={oninput_password}
                                    />
                                </fieldset>
                                <button
                                    class="btn btn-lg btn-primary pull-xs-right"
                                    type="submit"
                                    disable="false">
                                </button>
                            </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
