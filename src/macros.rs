
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
