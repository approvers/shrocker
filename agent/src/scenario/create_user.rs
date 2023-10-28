use crate::{cmd_exec::CommandExecutor, values::Username, context::Context};

pub async fn create_user<X>(ctx: &Context<X>, user: &Username, ssh_pub_key: &str)
    where X: CommandExecutor
{
    let user = user.to_string();

    println!("Creating a new user");
    ctx.cmd_exec.execute(&format!("useradd --create-home --groups undef_user {user}")).await.unwrap();
    ctx.cmd_exec.execute(&format!("passwd --delete {user}")).await.unwrap();

    let home_dir = format!("/home/{user}");

    ctx.cmd_exec.execute(&format!("mkdir {home_dir}/.ssh")).await.unwrap();
    ctx.cmd_exec.execute(&format!("echo '{ssh_pub_key}' > {home_dir}/.ssh/authorized_keys")).await.unwrap();
}