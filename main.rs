use clap::{Parser, Subcommand};
use std::error::Error;
use std::process;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Duration;

mod aircraft;
mod generator;
mod attacks;
mod export;
mod server;

#[derive(Parser, Debug)]
#[command(name = "ADS-B Ghost Injector")]
#[command(version = "0.2.0")]
#[command(author = "Soukaynarafik & Vanessalauransot")]
#[command(about = "Offensive simulation, real-time visualization and ADS-B signal injection tool", long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "adsb_output.csv")]
    output: String,

    #[arg(short, long, default_value_t = 15)]
    count: usize,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Spoof,
    Flood {
        #[arg(short, long, default_value_t = 100)]
        intensity: usize,
    },
    Live {
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    println!("==================================================");
    println!("ADS-B Ghost Injector v0.2");
    println!("Authors: Soukaynarafik & Vanessalauransot");
    println!("==================================================");

    if let Err(e) = run(cli).await {
        eprintln!("[ERROR] Critical failure during execution: {}", e);
        process::exit(1);
    }
}

async fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let mut traffic = generator::generate_traffic(cli.count);
    let cmd_clone = cli.command.clone();

    match &cli.command {
        Some(Commands::Spoof) | Some(Commands::Live { .. }) => {
            if let Some(Commands::Spoof) = &cli.command {
                println!("[WARN] Static Spoofing vectors initialized.");
            }
            let total = traffic.len();
            if total >= 3 {
                let chunk = total / 3;
                attacks::apply_position_spoofing(&mut traffic[0..chunk]);
                attacks::apply_impossible_speeds(&mut traffic[chunk..chunk*2]);
            } else {
                attacks::apply_position_spoofing(&mut traffic);
            }
        }
        Some(Commands::Flood { intensity }) => {
            println!("[WARN] Flood attack sequence triggered. Injecting {} ghost targets.", intensity);
            let mut ghost_traffic = attacks::generate_flood(*intensity);
            traffic.append(&mut ghost_traffic);
        }
        None => println!("[INFO] Nominal operation mode."),
    }

    if let Some(Commands::Live { port }) = cmd_clone {
        println!("[INFO] Starting real-time kinematic vector engine...");
        
        let shared_state = Arc::new(Mutex::new(traffic));
        let state_for_sim = Arc::clone(&shared_state);

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                let mut data = state_for_sim.lock().await;
                for ac in data.iter_mut() {
                    ac.update_position(1.0);
                }
            }
        });

        server::start_server(shared_state, port).await;
        return Ok(());
    }

    println!("[INFO] Writing telemetry payload to '{}'...", cli.output);
    export::export_to_csv(&cli.output, &traffic)?;
    println!("[SUCCESS] Execution completed. {} tracking vectors compiled.", traffic.len());

    Ok(())
}