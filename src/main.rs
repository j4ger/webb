const TARGETS: [&str; 5] = [
    "华中工学院",
    "华中科技大学",
    "华中理工大学",
    "同济",
    "武汉医学院",
];
pub const THROTTLE: usize = 2;
const ROOT: &str = "https://cn.govopendata.com";

use lazy_static::lazy_static;
use log::{info, trace, warn};
use std::sync::{atomic::AtomicU64, atomic::Ordering, mpsc::channel, Mutex};

mod log_expect;
use log_expect::LogExpect;

mod file_source;
use file_source::FileSource;

mod fetch;
use fetch::FetchClient;

mod parsers;
use parsers::*;

mod job_manager;
use job_manager::JobManager;

pub static COUNT: AtomicU64 = AtomicU64::new(0);
pub static SUCCESSES: AtomicU64 = AtomicU64::new(0);

lazy_static! {
    static ref SUCCESSLIST: Mutex<Vec<SuccessTarget>> = Mutex::new(Vec::new());
    static ref FAILLIST: Mutex<Vec<FailTarget>> = Mutex::new(Vec::new());
    static ref JOBMANAGER: Mutex<JobManager> = Mutex::new(JobManager::new());
    static ref FETCHCLIENT: FetchClient = FetchClient::new(THROTTLE);
}

fn main() {
    pretty_env_logger::init();

    let (tx, rx) = channel();
    let tx_clone = tx.clone();

    {
        let mut lock = JOBMANAGER.lock().log_expect("Failed to aquire lock.");
        lock.init(tx_clone);
    }

    ctrlc::set_handler(move || {
        info!("Received Ctrl-C event.");
        tx.send(()).log_expect("Could not send termination signal.");
    })
    .log_expect("Failed to set Ctrl-C handler.");

    let runtime = tokio::runtime::Runtime::new().log_expect("Failed to build Tokio runtime.");

    {
        let mut lock = JOBMANAGER.lock().log_expect("Failed to aquire lock.");
        lock.allocate();
    }

    runtime.spawn(index_task(
        "https://cn.govopendata.com/renminribao/".to_string(),
    ));

    info!("Waiting for Ctrl-C.");
    rx.recv().log_expect("Failed to listen for event.");

    runtime.shutdown_background();

    info!(
        "Processed {}/{}:",
        SUCCESSES.load(Ordering::Relaxed),
        COUNT.load(Ordering::Relaxed)
    );

    let success_list_content = SUCCESSLIST
        .lock()
        .log_expect("Failed to get data for SUCCESSLIST.");

    let fail_list_content = FAILLIST
        .lock()
        .log_expect("Failed to get data for FAILLIST.");

    info!("{:#?}", success_list_content);
    warn!("{:#?}", fail_list_content);

    let success_list = FileSource::new("success.yaml");
    let fail_list = FileSource::new("fail.yaml");

    success_list.store(&*success_list_content);
    fail_list.store(&*fail_list_content);
    info!("File saved.");
}

#[derive(serde::Serialize, Debug)]
pub struct SuccessTarget {
    title: String,
    url: String,
}

#[derive(serde::Serialize, Debug)]
enum TargetType {
    Index,
    Year,
    Month,
    Day,
    Article,
}

#[derive(serde::Serialize, Debug)]
struct FailTarget {
    url: String,
    target_type: TargetType,
}

#[derive(Debug)]
pub enum Error {
    NetworkError,
    ParsingError(String),
    GeneralError,
}

// dispatch year tasks
async fn index_task(url: String) {
    match FETCHCLIENT.fetch(&url).await {
        Ok(content) => match parse_index_page(content) {
            Ok(years) => {
                for year in years {
                    trace!("[index] Spawning task for {}.", &year);
                    {
                        let mut lock = JOBMANAGER.lock().log_expect("Failed to aquire lock.");
                        lock.allocate();
                    }

                    tokio::spawn(year_task(year));
                }
                {
                    let mut lock = JOBMANAGER.lock().log_expect("Failed to aquire lock.");
                    lock.deallocate();
                }
                return;
            }
            Err(error) => {
                warn!("[index] Parsing {} failed: {:?}.", url, error);
            }
        },
        Err(error) => {
            warn!("[index] Fetching {} failed: {:?}.", url, error);
        }
    }
    COUNT.fetch_add(1, Ordering::Relaxed);
    {
        let mut lock = FAILLIST.lock().log_expect("[index] Failed to aquire lock.");
        lock.push(FailTarget {
            url: url.to_string(),
            target_type: TargetType::Index,
        });
    }
    {
        let mut lock = JOBMANAGER.lock().log_expect("Failed to aquire lock.");
        lock.deallocate();
    }
}

// dispatch month tasks
async fn year_task(url: String) {
    match FETCHCLIENT.fetch(&url).await {
        Ok(content) => match parse_year_page(content) {
            Ok(months) => {
                for month in months {
                    trace!("[year] Spawning task for {}.", &month);
                    {
                        let mut lock = JOBMANAGER.lock().log_expect("Failed to aquire lock.");
                        lock.allocate();
                    }

                    tokio::spawn(month_task(month));
                }
                {
                    let mut lock = JOBMANAGER.lock().log_expect("Failed to aquire lock.");
                    lock.deallocate();
                }
                return;
            }
            Err(error) => {
                warn!("[year] Parsing {} failed: {:?}.", url, error);
            }
        },
        Err(error) => {
            warn!("[year] Fetching {} failed: {:?}.", url, error);
        }
    }
    COUNT.fetch_add(1, Ordering::Relaxed);
    {
        let mut lock = FAILLIST.lock().log_expect("[year] Failed to aquire lock.");
        lock.push(FailTarget {
            url: url.to_string(),
            target_type: TargetType::Year,
        });
    }
    {
        let mut lock = JOBMANAGER.lock().log_expect("Failed to aquire lock.");
        lock.deallocate();
    }
}

// dispatch day tasks
async fn month_task(url: String) {
    match FETCHCLIENT.fetch(&url).await {
        Ok(content) => match parse_month_page(content) {
            Ok(days) => {
                for day in days {
                    trace!("[month] Spawning task for {}.", &day);
                    {
                        let mut lock = JOBMANAGER.lock().log_expect("Failed to aquire lock.");
                        lock.allocate();
                    }

                    tokio::spawn(day_task(day));
                }
                {
                    let mut lock = JOBMANAGER.lock().log_expect("Failed to aquire lock.");
                    lock.deallocate();
                }
                return;
            }
            Err(error) => {
                warn!("[month] Parsing {} failed: {:?}.", url, error);
            }
        },
        Err(error) => {
            warn!("[month] Fetching {} failed: {:?}.", url, error);
        }
    }
    COUNT.fetch_add(1, Ordering::Relaxed);
    {
        let mut lock = FAILLIST.lock().log_expect("[month] Failed to aquire lock.");
        lock.push(FailTarget {
            url: url.to_string(),
            target_type: TargetType::Month,
        });
    }
    {
        let mut lock = JOBMANAGER.lock().log_expect("Failed to aquire lock.");
        lock.deallocate();
    }
}

// dispatch article tasks
/* async fn day_task(url: String) {
    match FETCHCLIENT.fetch(&url).await {
        Ok(content) => match parse_day_page(content) {
            Ok(articles) => {
                for article in articles {
                    trace!("[day] Spawning task for {}.", &article);
                    {
                        let mut lock = JOBMANAGER.lock().log_expect("Failed to aquire lock.");
                        lock.allocate();
                    }
                    tokio::spawn(article_task(article));
                }
                {
                    let mut lock = JOBMANAGER.lock().log_expect("Failed to aquire lock.");
                    lock.deallocate();
                }
                return;
            }
            Err(error) => {
                warn!("[day] Parsing {} failed: {:?}.", url, error);
            }
        },
        Err(error) => {
            warn!("[day] Fetching {} failed: {:?}.", url, error);
        }
    }
    COUNT.fetch_add(1, Ordering::Relaxed);
    {
        let mut lock = FAILLIST.lock().log_expect("[day] Failed to aquire lock.");
        lock.push(FailTarget {
            url: url.to_string(),
            target_type: TargetType::Day,
        });
    }
    {
        let mut lock = JOBMANAGER.lock().log_expect("Failed to aquire lock.");
        lock.deallocate();
    }
} */

// examine the article, test for keyword presence, save target to file
async fn day_task(url: String) {
    trace!("Processing day {}.", url);
    match FETCHCLIENT.fetch(&url).await {
        Ok(content) => match parse_day_page(content) {
            Ok(targets) => {
                for each_target in targets {
                    COUNT.fetch_add(1, Ordering::Relaxed);
                    for each in TARGETS {
                        if each_target
                            .title
                            .replace(['\n', '\t', '\r', ' '], "")
                            .contains(each)
                        {
                            info!(
                                "[article] Found keyword {} in article {}.",
                                each, &each_target.title
                            );
                            SUCCESSES.fetch_add(1, Ordering::Relaxed);
                            {
                                let mut lock = SUCCESSLIST
                                    .lock()
                                    .log_expect("[article] Failed to aquire lock.");
                                lock.push(each_target);
                            }
                            break;
                        }
                    }
                }
                {
                    let mut lock = JOBMANAGER.lock().log_expect("Failed to aquire lock.");
                    lock.deallocate();
                }
                return;
            }
            Err(error) => {
                warn!("[article] Parsing {} failed: {:?}.", url, error);
            }
        },
        Err(error) => {
            warn!("[article] Fetching {} failed: {:?}.", url, error);
        }
    }
    COUNT.fetch_add(1, Ordering::Relaxed);
    {
        let mut lock = FAILLIST
            .lock()
            .log_expect("[article] Failed to aquire lock.");
        lock.push(FailTarget {
            url: url.to_string(),
            target_type: TargetType::Article,
        });
    }
    {
        let mut lock = JOBMANAGER.lock().log_expect("Failed to aquire lock.");
        lock.deallocate();
    }
}
