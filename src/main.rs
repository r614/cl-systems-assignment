use structopt::StructOpt;
use url::Url;
mod handlers;

#[derive(StructOpt)]
#[structopt(
    name = "pingWorker",
    about = "Rust CLI helper to test Cloudfront worker performance.",
    author = "Roshan Pawar"
)]
struct Cli {
    /// The url to use, with format: https://google.com/images or https://r614.r614.workers.dev/links
    #[structopt(short = "u", long = "url")]
    url: String,

    #[structopt(short = "p", long = "profile", default_value = "0")]
    /// Number of times to ping the server and run a request.
    profile: i32,
}

fn main() {
    let args = Cli::from_args();
    let req_url = args.url.replace("www.", "");
    let parsed_url = Url::parse(&req_url).expect("Error validating URL: malformed URL entered.");
    if args.profile == 0 {
        println!("{}", handlers::make_request(&parsed_url));
    } else if args.profile < 0 {
        panic!("Profile cannot be a negative number")
    } else {
        handlers::measure_metrics(args.profile, &parsed_url);
    }
}
