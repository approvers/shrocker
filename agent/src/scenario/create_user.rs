use crate::{cmd_exec::CommandExecutor, values::{Username, SSHPublicKey}, context::HasContext};

pub enum CreateUserError {
    AlreadyExists,
    UserCreationFail(u8),
    UserInitFail(u8),
}

pub async fn create_user<X>(ctx: &impl HasContext<X>, user: &Username, ssh_pub_key: &SSHPublicKey) -> Result<(), CreateUserError>
    where X: CommandExecutor
{
    let ctx = ctx.ctx();
    let user = user.to_string();

    if ctx.cmd_exec.execute(&format!("grep '{user}' /etc/passwd")).await.is_ok() {
        return Err(CreateUserError::AlreadyExists);
    }

    ctx.cmd_exec
        .execute(&format!("useradd --create-home --groups undef_user {user}"))
        .await
        .map_err(CreateUserError::UserCreationFail)?;
    ctx.cmd_exec
        .execute(&format!("passwd --delete {user}"))
        .await
        .map_err(CreateUserError::UserCreationFail)?;

    let home_dir = format!("/home/{user}");

    ctx.cmd_exec
        .execute(&format!("mkdir {home_dir}/.ssh"))
        .await
        .map_err(CreateUserError::UserInitFail)?;
    ctx.cmd_exec
        .execute(&format!("echo '{}' > {home_dir}/.ssh/authorized_keys", ssh_pub_key.to_string()))
        .await
        .map_err(CreateUserError::UserInitFail)?;

    Ok(())
}