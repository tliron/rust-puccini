use floria_plugin_sdk::data::*;

/// The $greater_or_equal function takes two arguments of integer, float, string, timestamp,
/// version, any scalar type, or their derivations. It evaluates to true if both arguments are of
/// the same type, and if the first argument is greater than or equal to the second argument and
/// evaluates to false otherwise.
pub fn greater_or_equal(arguments: &Vec<Any>, site: &Site) -> Result<Any, String> {
    if arguments.len() != 2 {
        return Err("must have 2 arguments".into());
    }

    let left = arguments.first().expect("first argument");
    let right = arguments.get(1).expect("second argument");

    if let Some(tosca_data) = site.get_metadata_string("tosca:data")? {
        println!("{:?} {} {} {}", site.property_name, tosca_data, left.type_name(), right.type_name());
    }

    if !left.same_type(right) {
        return Err(format!("arguments must be of the same type: {}, {}", left.type_name(), right.type_name()));
    }

    Ok((left >= right).into())
}
