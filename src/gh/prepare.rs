

pub fn auth(token: String){
    let builder = octocrab::Octocrab::builder();
    octocrab::initialise(builder.personal_token(token).build().unwrap());
}