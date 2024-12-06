use anyhow::Ok;

pub mod prepare;

pub async fn all_repos(token: String, owner: String)-> anyhow::Result<String>{
    println!("{owner}");
    prepare::auth(token);
    let octocrab = octocrab::instance();
    let page: u32 = 0;
    let max: u8 = 100;
    let repos = octocrab.orgs(owner).list_repos().page(page).per_page(max).send().await.unwrap();
    let x = repos.items.iter().map(|repo| {
        repo.name.clone()
    }).collect::<Vec<String>>().join("\n");
    
    Ok(x.to_string())
}