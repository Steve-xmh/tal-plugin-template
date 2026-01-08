#[derive(Default)]
struct PluginTemplate {
    context: Option<tal_plugin_core::TalPluginContext>,
}

impl PluginTemplate {
    fn ctx(&self) -> &tal_plugin_core::TalPluginContext {
        self.context.as_ref().expect("Context not initialized")
    }
}

impl tal_plugin_core::TalPlugin for PluginTemplate {
    fn init(&mut self, ctx: tal_plugin_core::TalPluginContext) {
        self.context = Some(ctx);
    }
    
    // Override other necessary methods here
}

tal_plugin_core::define_plugin!(PluginTemplate);
