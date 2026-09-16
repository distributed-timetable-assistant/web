use crate::ApiError;
use crate::api::DitaClient;
use crate::hooks::auth::AuthState;
use dita_state::app_state::StateProvider;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub email: String,
    pub avatar_url: Option<String>,
}

impl UserInfo {
    pub fn display_name(&self) -> String {
        match (&self.first_name, &self.last_name) {
            (Some(f), Some(l)) if !f.is_empty() && !l.is_empty() => format!("{f} {l}"),
            (Some(f), _) if !f.is_empty() => f.clone(),
            (_, Some(l)) if !l.is_empty() => l.clone(),
            _ => self.username.clone().unwrap_or_else(|| self.email.clone()),
        }
    }

    pub fn initials(&self) -> String {
        let mut res = String::new();
        if let Some(f) = &self.first_name {
            if let Some(ch) = f.chars().next() {
                res.push(ch.to_ascii_uppercase());
            }
        }
        if let Some(l) = &self.last_name {
            if let Some(ch) = l.chars().next() {
                res.push(ch.to_ascii_uppercase());
            }
        }
        if res.is_empty() {
            if let Some(u) = &self.username {
                if let Some(ch) = u.chars().next() {
                    res.push(ch.to_ascii_uppercase());
                }
            } else if let Some(ch) = self.email.chars().next() {
                res.push(ch.to_ascii_uppercase());
            }
        }
        res
    }
}

impl From<AccountIdentity> for UserInfo {
    fn from(session: AccountIdentity) -> Self {
        Self {
            id: session.id,
            email: session.email,
            first_name: session.first_name,
            last_name: session.last_name,
            username: session.username,
            avatar_url: session.avatar_url,
        }
    }
}

#[derive(Clone, Copy)]
pub struct AccountState {
    user_info: LocalResource<Option<UserInfo>>,
}

impl AccountState {
    /// Obtains the `AccountState` from the Leptos context.
    ///
    /// Panics if `AccountState` was not registered via `AppStateBuilder::with::<AccountState>()`.
    pub fn new() -> Self {
        use_context::<AccountState>().expect(
            "AccountState: AccountState not found in context. \
             Did you call AppState::builder().with::<AccountState>()?",
        )
    }

    /// Access the reactive user info resource.
    pub fn user(&self) -> LocalResource<Option<UserInfo>> {
        self.user_info
    }

    /// Fetches the currently authenticated session and identity from Ory Kratos.
    pub async fn get_account_whoami() -> Result<AccountIdentity, ApiError> {
        let url = format!("{}/me", crate::config::account_url());
        let client = DitaClient::new();
        let resp = client.get(&url).await?;
        let session = resp.json::<AccountIdentity>().await?;
        Ok(session)
    }
}

impl StateProvider for AccountState {
    fn provide() {
        let auth = AuthState::new();
        let user_info = LocalResource::new(move || async move {
            if !auth.is_authenticated().get() {
                return None;
            }
            match AccountState::get_account_whoami().await {
                Ok(session) => Some(UserInfo::from(session)),
                Err(err) => {
                    web_sys::console::warn_1(
                        &format!("Failed to fetch Kratos account session: {:?}", err).into(),
                    );
                    None
                }
            }
        });
        provide_context(AccountState { user_info });
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountIdentity {
    /// Kratos Identity ID (equal to the OAuth `sub` claim).
    pub id: String,

    /// Primary account email address.
    pub email: String,

    /// First name from identity traits, if present.
    pub first_name: Option<String>,

    /// Last name from identity traits, if present.
    pub last_name: Option<String>,

    /// Username from identity traits, if present.
    pub username: Option<String>,

    /// Avatar URL from identity traits, if present.
    pub avatar_url: Option<String>,

    /// Kratos identity state (e.g. "active").
    pub state: Option<AccountIdentityState>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AccountIdentityState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
}
