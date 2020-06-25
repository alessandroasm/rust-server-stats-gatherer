struct HostSession<'a> {
    host_name: &'a str,
    host_config: &'a crate::config::AppHost,
}

impl<'a> HostSession<'a> {
    pub fn from_config(
        host_name: &'a str,
        host_config: &'a crate::config::AppHost,
    ) -> Self {
        HostSession {
            host_name,
            host_config,
        }
    }

    pub fn run(&self) -> crate::AppResult<()> {
        println!("Host: {}, config: {:?}", &self.host_name, &self.host_config);
        Ok(())
    }
}

pub fn run(config: &crate::config::AppConfig) -> crate::AppResult<()> {
    for (host, host_config) in config.hosts.iter() {
        println!("Key: {}, val: {:?}", &host, &host_config);
        let session = HostSession::from_config(&host, host_config);
        session.run()?;
    }
    Ok(())
}
