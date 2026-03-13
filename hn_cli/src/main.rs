use serde::Deserialize; //with a Z????

#[derive(Deserialize)] // parsing macro
struct Story {
    title: String,
    url: Option<String>, //is there no autocomplete :(
    score: u32,
    by: String
}

fn main() {
    println!("🔶 Top 10 Hacker News Stories\n");

    let client = reqwest::blocking::Client::new(); //sync slient

    let top_ids: Vec<u64> = client
    .get("https://hacker-news.firebaseio.com/v0/topstories.json")
    .send()
    .expect("Failed to fetch top stories")
    .json()
    .expect("Failed to parse story IDs");

    let emojis = ["1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "8️⃣", "9️⃣", "🔟"];

    for (i, id) in top_ids.iter().take(10).enumerate() {
        let url = format!("https://hacker-news.firebaseio.com/v0/item/{id}.json");

        let story: Story = client
            .get(&url)
            .send()
            .expect("Failed to fetch story")
            .json()
            .expect("Failed to parse story");

        let link = story.url.as_deref().unwrap_or("(no URL)");
        println!("{}. {} ({} points by {})", emojis[i], story.title, story.score, story.by);
        println!("   {}\n", link);
    }

}