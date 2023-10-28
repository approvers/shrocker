pub struct LocalCommandExecutor {
    ssh_key: String,
}

impl LocalCommandExecutor {
    pub fn new(ssh_key: &str) -> Self {
        Self { ssh_key: ssh_key.to_string()  }
    }
}


