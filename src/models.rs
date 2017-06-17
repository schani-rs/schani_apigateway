#[derive(Serialize, Deserialize, FromForm)]
pub struct Tag {
    pub id: i32,
    pub label: String,
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct NewTag {
    pub label: String,
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct RawImage {
    pub id: i32,
    pub user_id: i32,
    pub camera: String,
    pub latitude: f64,
    pub longitude: f64,
    //pub creation: DateTime<UTC>,
}

#[derive(Serialize,Deserialize, FromForm)]
pub struct NewRawImage {
    pub user_id: i32,
    pub camera: String,
    pub latitude: f64,
    pub longitude: f64,
    //pub creation: DateTime<UTC>,
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct Image {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub license: String,
    pub side_car_file: String,
    pub raw_image_id: i32,
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct NewImage {
    pub title: String,
    pub description: String,
    pub license: String,
    pub side_car_file: String,
    pub raw_image_id: i32,
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct Collection {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct NewCollection {
    pub name: String,
    pub description: String,
}
