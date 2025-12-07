use anyhow::Result;
use athena_api::server::ApiServer;
use athena_core::{config::AthenaConfig, system::AthenaSystem};
use clap::{Parser, Subcommand};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tracing_subscriber;

#[derive(Parser)]
#[command(name = "athena")]
#[command(about = "Athena OS - Decentralized Operating System")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, global = true)]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the Athena OS server
    Start {
        #[arg(long, default_value = "8080")]
        port: u16,
    },
    /// Initialize a new Athena OS instance
    Init {
        #[arg(long)]
        data_dir: Option<PathBuf>,
    },
    /// Create a new node in the graph
    CreateNode {
        #[arg(long)]
        label: String,
    },
    /// List all nodes
    ListNodes,
    /// Query the graph
    Query {
        #[arg(long)]
        pattern: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Start { port } => {
            let config = if let Some(config_path) = cli.config {
                AthenaConfig::load(config_path)?
            } else {
                AthenaConfig::default()
            };

            let system = Arc::new(AthenaSystem::new(config.clone()).await?);
            system.initialize().await?;

            // Start P2P synchronization if enabled
            if config.enable_p2p {
                system.start_p2p_sync().await?;
                tracing::info!("P2P synchronization enabled and started");
            }

            let server = ApiServer::new(system);
            let addr = SocketAddr::from(([127, 0, 0, 1], port));
            server.start(addr).await?;
        }
        Commands::Init { data_dir } => {
            let mut config = AthenaConfig::default();
            if let Some(dir) = data_dir {
                config.data_dir = dir;
                config.key_store_path = config.data_dir.join("keys.bin");
                config.graph_db_path = config.data_dir.join("graph");
            }

            std::fs::create_dir_all(&config.data_dir)?;
            let config_path = config.data_dir.join("config.toml");
            config.save(&config_path)?;

            println!("Initialized Athena OS at: {}", config.data_dir.display());
        }
        Commands::CreateNode { label } => {
            let config = if let Some(config_path) = cli.config {
                AthenaConfig::load(config_path)?
            } else {
                AthenaConfig::default()
            };

            let system = Arc::new(AthenaSystem::new(config).await?);
            system.initialize().await?;

            let entity = athena_graph::entity::Entity {
                id: athena_graph::entity::NodeId::new(),
                label,
                properties: std::collections::HashMap::new(),
                created_at: chrono::Utc::now().timestamp(),
                updated_at: chrono::Utc::now().timestamp(),
                version: 1,
            };

            system.graph_engine.put_node(entity.clone()).await?;
            println!("Created node: {}", entity.id.0);
        }
        Commands::ListNodes => {
            let config = if let Some(config_path) = cli.config {
                AthenaConfig::load(config_path)?
            } else {
                AthenaConfig::default()
            };

            let system = Arc::new(AthenaSystem::new(config).await?);
            system.initialize().await?;

            let pattern = athena_graph::query::GraphPattern {
                node_filters: vec![],
                edge_filters: vec![],
                limit: Some(100),
            };

            let result = system.graph_engine.query(&pattern).await?;
            println!("Found {} nodes:", result.nodes.len());
            for node in result.nodes {
                println!("  - {}: {}", node.id.0, node.label);
            }
        }
        Commands::Query { pattern: _ } => {
            println!("Query functionality coming soon");
        }
    }

    Ok(())
}

