use clap::{Parser, Subcommand};
use std::error::Error;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use std::io::Read;
use serde::Serialize;

#[derive(Debug,Serialize)]
struct Commit {
    hash:String,
    author:String,
    date:String,
    message:String
}

#[derive(Debug,Serialize)]
struct Branch {
    name:String,
    commits :Vec<Commit>
}

#[derive(Parser)]
#[command(author="Danish" , version="0.1", about="Ledger Explorer CLI")]
struct Cli {
    #[command(subcommand)]
    command:Commands
}

#[derive(Subcommand)]
enum Commands {
    ViewCommits {
        path:String
    },
    ListBranches {
        path:String
    },
    ExportJson {
        path:String
    }
}

fn main()-> Result<(),Box<dyn Error>> {
    let s = String::new();
    
    let cli = Cli :: parse();
    match &cli.command {
        Commands :: ViewCommits { path } => {
            let commits = parse_commits(path)?;
            for commit in commits {
                println!("{} | {} | {} ",commit.hash,commit.author , commit.date);
                println!("message : {}",commit.message)
            }
        },
        Commands :: ListBranches {path} => {
            let branches = parse_branch(path)?;
            for branch in branches {
                println!("Branch : {}",branch.name);
            }
        },
        Commands :: ExportJson { path } => {
            let commits = parse_commits(path)?;
            let json = serde_json::to_string_pretty(&commits)?;
            println!("Json data : {}",json)
        }
    }
    Ok(())
}
fn parse_commits (repo_path:&str)->Result<Vec<Commit>,Box<dyn Error>>{
    let git_objects = Path::new(repo_path).join(".git/objects");
    if !git_objects.exists() {
        return Err("Invalid repo path or .git folder missing".into())
    }
    let mut commits = Vec::new();
    println!("raw paths:{}",repo_path);
    println!("converted path:{}",git_objects.display());
    for entry in WalkDir::new(&git_objects){
        println!("each entry {:?}",entry);
        let entry = entry?;
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(hash) = path.file_name() {
                println!("\n \n hash{:?}",hash);
                let hash_str = hash.to_string_lossy().to_string();
                commits.push(Commit {
                    hash:hash_str,
                    author:"unknown".to_string(),
                    date:"unknown".to_string(),
                    message:"unknown".to_string()
                })
            }
        }
    }
    Ok(commits)
}

fn parse_branch(str_path:&str)->Result<Vec<Branch>,Box<dyn Error>>{
    let refs_path = Path::new(str_path).join(".git/refs/heads");
    if !refs_path.exists(){
        return Err(".git/refs/heads folder missing!".into())
    }
    // println!("refs_path : {:?}",refs_path);
    let mut branches = Vec::new();
    for entry in fs::read_dir(refs_path) ?{
        let entry = entry?;
        let branch_name = entry.file_name().to_string_lossy().to_string();
        branches.push( Branch {
            name:branch_name,
            commits:vec![]
        })
    }
    Ok(branches)
}
