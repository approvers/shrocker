use crate::{context::HasContext, cmd_exec::CommandExecutor, values::SSHPublicKey};

pub enum KeyValidationError {
    Malformed,
    PrivateKey
}

pub async fn validate_key<'ctx, X>(ctx: &impl HasContext<X>, ssh_pub_key: &str) -> Result<SSHPublicKey, KeyValidationError>
    where X: CommandExecutor + 'ctx
{
    let ctx = ctx.ctx();

    if ssh_pub_key.contains('\'') {
        return Err(KeyValidationError::Malformed);
    }

    if ctx.cmd_exec.execute(&format!("ssh-keygen -l -f <(echo '{ssh_pub_key}')")).await.is_err() {
        return Err(KeyValidationError::Malformed);
    }

    if ssh_pub_key.contains("PRIVATE") {
        return Err(KeyValidationError::PrivateKey);
    }

    Ok(SSHPublicKey::new(ssh_pub_key))
}
