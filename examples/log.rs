use supercluster::{CoordinateSystem, FeatureBuilder, Supercluster, SuperclusterError};

fn main() -> Result<(), SuperclusterError> {
    // Initialize the logger
    env_logger::init();

    // Create a list of features
    let features = FeatureBuilder::new()
        .add_point(vec![-77.032, 38.913])
        .add_point(vec![-77.033, 38.913])
        .add_point(vec![-77.034, 38.913])
        .build();

    // Set the configuration settings
    let options = Supercluster::builder()
        .radius(40.0)
        .extent(512.0)
        .min_points(2)
        .max_zoom(16)
        .coordinate_system(CoordinateSystem::LatLng)
        .build();

    // Create a new instance with the specified configuration settings
    let mut cluster = Supercluster::new(options);

    // Load features into the Supercluster instance
    let index = cluster.load(features)?;

    // Get a tile from the Supercluster instance
    index.get_tile(0, 0.0, 0.0)?;

    Ok(())
}
