
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

#[macro_export]
macro_rules! repeat_type {
    ($name:ident, $type:ty) => {
        $name($type),
        $name$name($type, $type),
        $name$name$name($type, $type, $type),
    };
}

#[macro_export]
macro_rules! create_from_type_for_gmdata1 {
    ($type:ty, $name:ident) => {
        impl From<$type> for GMData {
            fn from(data: $type) -> Self {
                GMData::$name(data)
            }
        }
    };
}

#[macro_export]
macro_rules! create_from_type_for_gmdata2 {
    ($type:ty, $name:ident) => {
        impl From<($type, $type)> for GMData {
            fn from(data: ($type, $type)) -> Self {
                GMData::$name(data.0, data.1)
            }
        }
    };
}

#[macro_export]
macro_rules! create_from_type_for_gmdata3 {
    ($type:ty, $name:ident) => {
        impl From<($type, $type, $type)> for GMData {
            fn from(data: ($type, $type, $type)) -> Self {
                GMData::$name(data.0, data.1, data.2)
            }
        }
    };
}

#[macro_export]
macro_rules! create_from_type_for_gmdata_all {
    ($type:ty, $name1:ident, $name2:ident, $name3:ident) => {
        create_from_type_for_gmdata1!($type, $name1);
        create_from_type_for_gmdata2!($type, $name2);
        create_from_type_for_gmdata3!($type, $name3);
    };
}



#[macro_export]
macro_rules! create_from_gmdata_for_type1 {
    ($type:ty, $name:ident) => {
        impl From<GMData> for $type {
            fn from(data: GMData) -> Self {
                if let GMData::$name(data) = data {
                    data
                } else {
                    error_panic(&format!("Expected $name, got {:?}", data))
                }
            }
        }
    };
}

#[macro_export]
macro_rules! create_from_gmdata_for_type2 {
    ($type:ty, $name:ident) => {
        impl From<GMData> for ($type, $type) {
            fn from(data: GMData) -> Self {
                if let GMData::$name(data1, data2) = data {
                    (data1, data2)
                } else {
                    error_panic(&format!("Expected $name, got {:?}", data))
                }
            }
        }
    };
}

#[macro_export]
macro_rules! create_from_gmdata_for_type3 {
    ($type:ty, $name:ident) => {
        impl From<GMData> for ($type, $type, $type) {
            fn from(data: GMData) -> Self {
                if let GMData::$name(data1, data2, data3) = data {
                    (data1, data2, data3)
                } else {
                    error_panic(&format!("Expected $name, got {:?}", data))
                }
            }
        }
    };
}

#[macro_export]
macro_rules! create_from_gmdata_for_type_all {
    ($type:ty, $name1:ident, $name2:ident, $name3:ident) => {
        create_from_gmdata_for_type1!($type, $name1);
        create_from_gmdata_for_type2!($type, $name2);
        create_from_gmdata_for_type3!($type, $name3);
    };
}

#[macro_export]
macro_rules! create_builder_methods {
    ($builder:ident, $base:ty, $member:ident) => {
        pub fn with_active(mut self, active: bool) -> Self {
            debug!("{}::with_active(), active: '{}'", "$builder", active);
    
            self.$member.base.active = active;
            self
        }
    
        pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
            let name = name.into();
            debug!("{}::with_name(), name: '{}'", "$builder", name);
    
            self.$member.base.name = name;
            self
        }
    
        pub fn with_group<S: Into<String>>(mut self, group: S) -> Self {
            let group = group.into();
            debug!("{}::with_group(), group: '{}'", "$builder", group);
    
            self.$member.base.groups.insert(group);
            self
        }
    
        pub fn with_groups(mut self, groups: HashSet<String>) -> Self {
            debug!("{}::with_groups(), groups: '{:?}'", "$builder", groups);
    
            self.$member.base.groups = groups;
            self
        }
    
        pub fn with_effect<T: 'static + GMEffectT<$base>>(mut self, effect: T) -> Self {
            debug!("{}::with_effect()", "$builder");
    
            self.$member.effects.add_effect(effect);
            self
        }
    
        pub fn with_effect2(mut self, effect: Box<dyn GMEffectT<$base>>) -> Self {
            debug!("{}::with_effect2()", "$builder");
    
            self.$member.effects.add_effect2(effect);
            self
        }
    
        pub fn with_effects(mut self, effects: Vec<Box<dyn GMEffectT<$base>>>) -> Self {
            debug!("{}::with_effects()", "$builder");
    
            self.$member.effects.set_effects(effects);
            self
        }
    };
}
