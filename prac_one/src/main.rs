use std::env;
use rand::Rng;

// Richard Lastrucci - u20430168
// Thabo Chesane - u20507102

fn main() {
    let args: Vec<_> = env::args().collect();

    let out = if args.len() == 1 {
      default()
    } else {
        match args[1].as_str() {
            "right" => right(),
            "wrong" => wrong(),
            _ => default(),
        }
    };

    println!("{}", out);
}

fn default() -> String{
  let mut rng = rand::thread_rng();
  let num_1 = rng.gen_range(0..9999);
  let num_2 = rng.gen_range(0..99999);

  let divs = if num_1 > num_2 {
    format!(r#"
    <div class = "main_div">
      <div class = "message">
        <a href="./prac_one?right">{}</a>
      </div>
      <div class = "message">
        <a href="./prac_one?wrong">{}</a>
      </div>w
    </div>
    "#, num_1, num_2)
  } else {
    format!(r#"
    <div class = "main_div">
      <div class = "message">
        <a href="./prac_one?wrong">{}</a>
      </div>
      <div class = "message">
        <a href="./prac_one?right">{}</a>
      </div>
    </div>
    "#, num_1, num_2)
  };

  let html = format!(r#"
    <!DOCTYPE html>
    <html>
      <head>
        <meta charset="UTF-8">./style.css">
        <title>COS 332 Prac One</title>
      </head>
      <body>
        <header>
          <h1>Choose a larger number</h1>
        </header>
        <main>
        {}
        </main>
        <footer>
          <p class="footer left"> Richard Lastrucci - 20430168 / Thabo Chesane - u20507102 </p>
          <p class="footer right"> COS 332</p>
        </footer>
      </body>
    </html>
            "#, divs);

  format!(
      "Content-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
      html.len(),
      html
  )
}


fn right() -> String {
    let html = r#"
    <!DOCTYPE html>
  <html>
    <head>
      <meta charset="UTF-8">
      <link rel="stylesheet" href="~/COS_332_pracs/prac_one/src/style.css">
      <title>COS 332 Prac One</title>
    </head>
    <body>
      <header>
        <h1>Choose a larger number</h1>
      </header>
      <main>
      <div class = "main_div">
        <div class="message right">
          <div class>Congrats! You can perform "greater than"</div>
        </div>
        <div class = "message">
            <a href="./prac_one"> Try Again! </a>
        </div>
      <div>
      </main>
      <footer>
        <p class="footer left"> Richard Lastrucci - 20430168 / Thabo Chesane - u20507102</p>
        <p class="footer right"> COS 332</p>
      </footer>
    </body>
  </html>
            "#;

    format!(
        "Content-type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        html.len(),
        html
    )
}

fn wrong() -> String{
    let html = r#"
    <!DOCTYPE html>
  <html>
    <head>
      <meta charset="UTF-8">
      <link rel="stylesheet" href="~/COS_332_pracs/prac_one/src/style.css">
      <title>COS 332 Prac One</title>
    </head>
    <body>
      <header>
        <h1>Choose a larger number</h1>
      </header>
      <main>
      <div class = "main_div">
        <div class="message wrong">
          <div>Oh dear! You can't complete a basic task...</div>
        </div>
        <div class="message">
          <div>
            <a href="./prac_one"> Try Again! </a>
          </div>
        </div>
      <div>
      </main>
      <footer>
        <p class="footer left"> Richard Lastrucci - 20430168 / Thabo Chesane - u20507102 </p>
        <p class="footer right"> COS 332</p>
      </footer>
    </body>
  </html>
            "#;

    format!(
        "Content-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        html.len(),
        html
    )
}

