use anyhow::Ok;
use mestats_model::Contributions;
use mestats_model::Stats;

pub mod prepare;

pub async fn get_contributors(
    token: String,
    owner: String,
    user_name: String,
) -> anyhow::Result<String> {
    println!("{owner}");
    prepare::auth(token);
    let octocrab = octocrab::instance();
    let page: u32 = 0;
    let max: u8 = 100;
    let repos = octocrab
        .orgs(owner.clone())
        .list_repos()
        .page(page)
        .per_page(max)
        .send()
        .await
        .unwrap();

    let mut cs: Vec<Contributions> = vec![];

    for repo in repos.items.iter() {
        let cso = octocrab
            .repos(owner.clone(), repo.name.clone())
            .list_contributors()
            .page(page)
            .per_page(max)
            .send()
            .await
            .unwrap()
            .items;

        let lang = octocrab
            .repos(owner.clone(), repo.name.clone())
            .list_languages()
            .await
            .unwrap_or_default();

        for contributor in cso.iter() {
            cs.push(Contributions::new(
                contributor.author.login.clone(),
                repo.name.clone(),
                contributor.contributions.clone(),
                lang.clone(),
            ));
        }
    }

    let only_user = cs
        .iter()
        .cloned()
        .filter(|c| c.login() == &user_name)
        .collect::<Vec<Contributions>>();
    let s = Stats::new(only_user).as_toml();

    Ok(s)
}
