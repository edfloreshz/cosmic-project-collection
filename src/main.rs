use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Application {
    name: String,
    description: String,
    repo: String,
    image: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Applications {
    list: Vec<Application>,
}

fn main() -> anyhow::Result<()> {
    let data = include_str!("../data/applications.ron");
    let mut readme = String::new();
    let applications: Applications = ron::from_str(data)?;

    readme.push_str("# COSMIC Application Collection\n");
    readme.push_str("A collection of COSMIC applications developed by the community.\n\n");

    readme.push_str("| Name | Description | Image |\n");
    readme.push_str("|---|---|---|\n");

    for app in applications.list {
        let row = format!("| [{}]({}) | {} | <img src=\"{}\" alt=\"{}\" width=\"200\"/> |\n", app.name, app.repo, app.description, app.image, app.name);
        readme.push_str(&row);
    }

    readme.push_str("## How to add your applet?\n");
    readme.push_str("To add your applet to this list, please open a pull request with your applet added to the `applets.ron` file and an image of your applet in the `img` folder named after your applet.\n");

    std::fs::write("README.md", readme)?;

    Ok(())
}
