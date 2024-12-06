use std::fs;

use mestats_model::Contributions;
use mestats_model::Stats;

pub mod prepare;

pub async fn get_all_contributions(
    token: String,
    anonymous: Vec<String>,
) -> anyhow::Result<String> {
    prepare::auth(token);
    let octocrab = octocrab::instance();
    let user_name = octocrab.current().user().await?.login;
    let orgs = octocrab
        .current()
        .list_org_memberships_for_authenticated_user()
        .send()
        .await
        .unwrap();
    let mut cos: Vec<Contributions> = vec![];

    println!("Fetching contributions for {}", user_name);

    for org in orgs.items {
        println!("Fetching contributions to {:?}", org.organization.login);
        let org = org.organization.login;
        let pot_cos = get_contributors(org, user_name.clone()).await;
        if let Ok(mut actual) = pot_cos {
            cos.append(&mut actual);
        }
    }

    let anon = cos
        .iter()
        .map(|c| {
            if anonymous.contains(c.owner()) {
                c.anonymize()
            } else {
                c.clone()
            }
        })
        .collect::<Vec<Contributions>>();

    let toml_string = Stats::new(anon).as_toml();

    fs::write("stats.toml", toml_string)?;

    Ok("Got all contributions".to_string())
}

async fn get_contributors(owner: String, user_name: String) -> anyhow::Result<Vec<Contributions>> {
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
            .unwrap_or_default()
            .items;

        let lang = octocrab
            .repos(owner.clone(), repo.name.clone())
            .list_languages()
            .await
            .unwrap_or_default();

        for contributor in cso.iter() {
            cs.push(Contributions::new(
                contributor.author.login.clone(),
                owner.clone(),
                repo.name.clone(),
                contributor.contributions,
                lang.clone(),
            ));
        }
    }

    let only_user = cs
        .into_iter()
        .filter(|c| c.login() == &user_name)
        .collect::<Vec<Contributions>>();

    Ok(only_user)
}
