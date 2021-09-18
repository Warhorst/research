use futures::executor::block_on;

async fn learn_song() -> usize {
    println!("learn");
    5
}

async fn sing_song(song: usize) {
    println!("sing");
}

async fn dance() {
    println!("dance");
}

/// Warte asynchron, ohne den Thread zu blockieren
async fn learn_and_sing_non_blocking() {
    let song = learn_song().await;
    sing_song(song).await;
}

/// Blockiere den thread um zu warten
async fn learn_and_sing_blocking() {
    let song = block_on(learn_song());
    block_on(sing_song(song));
}

pub async fn run() {
    let f1 = learn_and_sing_non_blocking();
    let f2 = dance();
    futures::join!(f1, f2);
}

