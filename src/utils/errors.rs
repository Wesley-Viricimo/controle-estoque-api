#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum Error {
    #[error("sea_orm::DbErr: {0}")]
    Db(#[from] sea_orm::DbErr),

    #[error("ENV VARIABLE for `{0}` is not set")]
    EnvironmentVariableNotSet(String),

    #[error("Resource Not Found: {0}")]
    NotFound(String),

    #[error("{0}")]
    ServerStartFailed(#[from] std::io::Error),

    #[error("InvalidUuid: UUID {0} is not valid")]
    InvalidUuid(String),

    #[error("Erro ao executar consulta: {0}")]
    DatabaseError(String),
}