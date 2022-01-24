use core::fmt;
use std::ops::Deref;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{services::set_token, types::UserInfo, AppRoute};

pub struct UseUserContextHandle {
    inner: UseStateHandle<UserInfo>,
    history: AnyHistory,
}

impl UseUserContextHandle {
    pub fn login(&self, value: UserInfo) {
        set_token(Some(value.token.clone()));
        self.inner.set(value);
        self.history.push(AppRoute::Home);
    }

    pub fn logout(&self) {
        set_token(None);
        self.inner.set(UserInfo::default());
        self.history.push(AppRoute::Home);
    }
}

impl Deref for UseUserContextHandle {
    type Target = UserInfo;

    fn deref(&self) -> &Self::Target {
        &(*self.inner)
    }
}

impl Clone for UseUserContextHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            history: self.history.clone(),
        }
    }
}

impl PartialEq for UseUserContextHandle {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl fmt::Debug for UseUserContextHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseUserContextHandle")
            .field("value", &format!("{:?}", *self.inner))
            .finish()
    }
}

pub fn use_user_content() -> UseUserContextHandle {
    let inner = use_context::<UseStateHandle<UserInfo>>().unwrap();
    let history = use_history().unwrap();

    UseUserContextHandle { inner, history }
}
