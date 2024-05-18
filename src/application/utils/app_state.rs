use async_session::MemoryStore;
use axum::extract::FromRef;
use azure_identity::authorization_code_flow::AuthorizationCodeFlow;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub store: MemoryStore,
    pub auth: MyAuth,
    pub db_pool: PgPool,
}

pub struct MyAuth {
    pub code_flow: AuthorizationCodeFlow,
}

impl FromRef<AppState> for MemoryStore {
    fn from_ref(state: &AppState) -> Self {
        state.store.clone()
    }
}

impl FromRef<AppState> for MyAuth {
    fn from_ref(state: &AppState) -> Self {
        state.auth.clone()
    }
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.db_pool.clone()
    }
}

// pkce_code_verifier doesn't support clone so we have to recreate the entire AuthorizationCodeFlow.
impl Clone for MyAuth {
    fn clone(&self) -> Self {
        MyAuth {
            code_flow: AuthorizationCodeFlow {
                authorize_url: self.code_flow.authorize_url.clone(),
                client: self.code_flow.client.clone(),
                csrf_state: oauth2::CsrfToken::new(self.code_flow.csrf_state.secret().clone()),
                pkce_code_verifier: oauth2::PkceCodeVerifier::new(
                    self.code_flow.pkce_code_verifier.secret().clone(),
                ),
            },
        }
    }
}
