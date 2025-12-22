use trpl::{Either, Html, join}; 
use std::{thread, time::Duration};

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response = trpl::get(url).await; 
    let response_text = response.text().await; 

    let title = Html::parse(&response_text)
    .select_first("title")
    .map(|title| title.inner_html());
    (url, title)
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{name} ran for {ms}ms ")
}

fn main() {


    // trpl::block_on(async {

    //     let a = async {
    //         println!("A1");
    //         trpl::sleep(Duration::from_secs(1)).await;
    //         println!("A2");
    //     };

    //     let b = async {
    //         println!("B1");
    //         println!("B2");
    //     };

    //     join(a, b).await


    // });



    trpl::block_on(async {

    let (tx, mut rx) = trpl::channel();

    let tx1 = tx.clone();

    let tx1_fut = async move {
            let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

    for val in vals {
            tx1.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let rx_fut = async {
         while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }


    };

    let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };


    trpl::join!(tx_fut, tx1_fut, rx_fut);

    })


    // trpl::block_on(async {

    //     let fut1 = async {
    //     for i in 1..10 {
    //         println!("hi number {i} from the first task");
    //         trpl::sleep(Duration::from_millis(500)).await;
    //     }
    // };

    // let fut2 = async {
    //     for i in 1..5 {
    //         println!("hi number {i} from the second task");
    //         trpl::sleep(Duration::from_millis(500)).await;
    //     }
    // };

    //     trpl::join(fut1, fut2).await;

    // });


    // let args: Vec<String> = std::env::args().collect();
    // trpl::block_on(async {
    //     let title_fut_1 = page_title(&args[1]);
    //     let title_fut_2 = page_title(&args[2]);


    //     let (url, maybe_title) = 
    //     match trpl::select(title_fut_1, title_fut_2).await {
    //         Either::Left(left) => left,
    //         Either::Right(right) => right,
    //     };

    //     println!("{url} returned first");
    //     match maybe_title {
    //         Some(title) => println!("its page title was: {title}"),
    //         None => println!("it had no title"),
    //     }

    // })

    // trpl::block_on(async {
    //     let handle = trpl::spawn_task(async {
    //         for i in 1..10 {
    //             println!("hi number {i} from the first task!");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     });

    //     for i in 1..5 {
    //         println!("hi number {i} from the second task");
    //         trpl::sleep(Duration::from_millis(500)).await;
    //     }

    //     handle.await.unwrap();

    // });


}