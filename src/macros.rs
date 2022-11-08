
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
macro_rules! create_from_type_for_gmdata {
    ($type:ty, $name1:ident) => {
        impl From<$type> for GMData {
            fn from(data: $type) -> Self {
                GMData::$name1(data)
            }
        }
    };

    ($type:ty, $name1:ident, $name2:ident) => {
        create_from_type_for_gmdata!($type, $name1);

        impl From<($type, $type)> for GMData {
            fn from(data: ($type, $type)) -> Self {
                GMData::$name2(data.0, data.1)
            }
        }
    };

    ($type:ty, $name1:ident, $name2:ident, $name3:ident) => {
        create_from_type_for_gmdata!($type, $name1, $name2);
        
        impl From<($type, $type, $type)> for GMData {
            fn from(data: ($type, $type, $type)) -> Self {
                GMData::$name3(data.0, data.1, data.2)
            }
        }        
    };
}

#[macro_export]
macro_rules! create_from_gmdata_for_type {
    ($type:ty, $name1:ident) => {
        impl From<GMData> for $type {
            fn from(data: GMData) -> Self {
                if let GMData::$name1(data) = data {
                    data
                } else {
                    error_panic(&format!("Expected $name, got {:?}", data))
                }
            }
        }    
    };

    ($type:ty, $name1:ident, $name2:ident) => {
        create_from_gmdata_for_type!($type, $name1);

        impl From<GMData> for ($type, $type) {
            fn from(data: GMData) -> Self {
                if let GMData::$name2(data1, data2) = data {
                    (data1, data2)
                } else {
                    error_panic(&format!("Expected $name2, got {:?}", data))
                }
            }
        }        
    };

    ($type:ty, $name1:ident, $name2:ident, $name3:ident) => {
        create_from_gmdata_for_type!($type, $name1, $name2);

        impl From<GMData> for ($type, $type, $type) {
            fn from(data: GMData) -> Self {
                if let GMData::$name3(data1, data2, data3) = data {
                    (data1, data2, data3)
                } else {
                    error_panic(&format!("Expected $name3, got {:?}", data))
                }
            }
        }        
    };
}
