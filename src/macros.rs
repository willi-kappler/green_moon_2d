
#[macro_export]
macro_rules! return_name_and_groups {
    () => {
        fn name(&self) -> &str {
            &self.name
        }
    
        fn groups(&self) -> &HashSet<String> {
            &self.groups
        }    
    };
}
