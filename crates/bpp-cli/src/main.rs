use std::fs;
use structopt::StructOpt;
use uuid::Uuid;

#[derive(StructOpt, Debug)]
pub struct BppCli {
    #[structopt(subcommand)]
    subcommands: SubCommands,
}

#[derive(StructOpt, Debug)]
enum SubCommands {
    #[structopt(about = "Add new note")]
    Add(AddOpts),
    #[structopt(about = "Remove existing note")]
    Rm(RmOpts),
    #[structopt(about = "Search for note")]
    Search(SearchOpts)
}

#[derive(StructOpt, Debug)]
struct AddOpts {
    #[structopt(
        long,
        short="t",
        long_help="Note Title",
        required_unless_one(&["edit"]),
        conflicts_with_all(&["edit"])
    )]
    title: Option<String>,
    #[structopt(
        long,
        short="c",
        long_help="NoTe Content",
        required_unless_one(&["edit"]),
        conflicts_with_all(&["edit"])
    )]
    content: Option<String>,
    #[structopt(
        long,
        short="e",
        long_help="Edit using VIM",
        required_unless_one(&["title", "content"]),
        conflicts_with_all(&["title", "content"])
    )]
    edit: bool
}

#[derive(StructOpt, Debug)]
struct RmOpts {
    #[structopt(
    long,
    short="i",
    long_help="Note Id to be removed",
    )]
    id: String
}

#[derive(StructOpt, Debug)]
struct SearchOpts {
    #[structopt(
    long,
    short="a",
    long_help="Search both to title and content",
    )]
    all: bool,

    #[structopt()]
    input: String
}

#[derive(Debug)]
pub enum BppCliError {

}

impl BppCli {
    pub fn run(&self) -> Result<i32, BppCliError> {
        match &self.subcommands {
            SubCommands::Add(add_opts) => {
                if add_opts.edit {
                    let temp_file = format!("/tmp/note_{:}.bpp", Uuid::new_v4());
                    let cmd = format!("vim {temp_file}");
                    std::process::Command::new("/bin/sh")
                        .arg("-c")
                        .arg(&cmd)
                        .spawn()
                        .expect("Error: Failed to start VIM") // TODO: handle errors
                        .wait()
                        .expect("Error: Editor Crashed"); // TODO: handle errors
                    let user_text = fs::read_to_string(temp_file).expect("Error to read temp file");// TODO: handle errors
                    println!("Add with edit {:?}", user_text);
                } else {
                    println!("Add with otions {:?}", add_opts);
                }
            }
            SubCommands::Rm(rm_opts) => { println!("Rm with otions {:?}", rm_opts); }
            SubCommands::Search(search_opts) => { println!("Search with otions {:?}", search_opts); }
        }

        Ok(0)
    }
}

fn main() {
    let cli = BppCli::from_args();
    match cli.run() {
        Ok(_) => (),
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
