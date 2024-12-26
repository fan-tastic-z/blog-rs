use std::{fmt, sync::Arc};

use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    Pool, Postgres,
};
use sqlx_adapter::{
    casbin::{self, CoreApi, DefaultModel, Enforcer, MgmtApi},
    SqlxAdapter,
};
use tokio::sync::RwLock;

use crate::config::DatabaseSettings;

const ACL_MODEL: &str = r#"
[request_definition]
r = sub, obj, act

[policy_definition]
p = sub, obj, act

[policy_effect]
e = some(where (p.eft == allow))

[matchers]
m = r.sub == p.sub && keyMatch(r.obj, p.obj) && regexMatch(r.act, p.act)
"#;

#[derive(Debug, Clone)]
pub struct Pg {
    pub pool: Pool<Postgres>,
    pub enforcer: EnforcerWrapper,
}

#[derive(Clone)]
pub struct EnforcerWrapper(Arc<RwLock<Enforcer>>);

impl EnforcerWrapper {
    pub fn new(enforcer: Enforcer) -> Self {
        Self(Arc::new(RwLock::new(enforcer)))
    }

    pub async fn add_policy(&self, ptype: &str, params: Vec<String>) -> anyhow::Result<()> {
        self.0.write().await.add_named_policy(ptype, params).await?;
        Ok(())
    }
}

impl fmt::Debug for EnforcerWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EnforcerWrapper")
            .field("enforcer", &"Enforcer")
            .finish()
    }
}

impl Pg {
    pub async fn new(config: DatabaseSettings) -> anyhow::Result<Self, anyhow::Error> {
        let opts = PgConnectOptions::new()
            .host(&config.host)
            .port(config.port)
            .username(&config.username)
            .password(&config.password)
            .database(&config.database_name);
        let pool = PgPoolOptions::new().connect_with(opts).await?;
        let model = DefaultModel::from_str(ACL_MODEL).await?;
        let adapter = SqlxAdapter::new_with_pool(pool.clone()).await?;
        let enforcer = casbin::Enforcer::new(model, adapter).await?;
        Ok(Self {
            pool,
            enforcer: EnforcerWrapper::new(enforcer),
        })
    }
}
