/// https://docs.rs/inline-c/latest/inline_c/#
///
/// Hello, Gnostr!
///
/// # Example
///
/// ```rust
/// # use inline_c::assert_c;
/// #
/// # fn main() {
/// #     (assert_c! {
/// #include <stdio.h>
///
/// int main() {
///     printf("Hello, Gnostr!");
///
///     return 0;
/// }
/// #    })
/// #    .success()
/// #    .stdout("Hello, Gnostr!");
/// # }
/// ```

extern crate getopts;
use getopts::Options;
use git2::{Repository, Revwalk, Commit};
extern crate gnostr_xq;
use gnostr_xq::xq_lang;

//use nostr_types::Event;
use std::env;
//use std::io::Read;
use std::process;

use inline_c::assert_c;
//use gnostr_bins;

use std::process::{Command, Stdio};

//mod reflog;
mod reflog_simple;

//inline module get_self_source
mod get_self_source {
use std::io::{Read};
use reqwest::Url;
//TODO:this will be refactored as an nostr EVENT query
  pub fn get_self(){
      let url = Url::parse("https://raw.githubusercontent.com/gnostr-org/gnostr/master/src/main.rs").unwrap();
      let mut res = reqwest::blocking::get(url).unwrap();

      let mut body = String::new();
      res.read_to_string(&mut body).unwrap();

      println!("{}", body);
  }
}//end inline module get_self_source
//use inline module get_self_source
pub use get_self_source::get_self;

fn gen_keys(){

use k256::schnorr::SigningKey;
use rand_core::OsRng;

    let _signing_key = SigningKey::random(&mut OsRng);
    let _verifying_key = _signing_key.verifying_key();
    //println!("PUBLIC: {:x}", verifying_key.to_bytes());
    //we only return _signing_key
    //so it may be streamed into other utilities
    println!("{:x}", _signing_key.to_bytes());

}

fn set_kind(){

  let args_vector: Vec<String> = env::args().collect();
  println!("args_vector.len()={:x}", args_vector.len());

}

//COMMAND:
//CONTEXT
//gnostr --sec e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | gnostr-post-event --relay wss://relay.damus.io
#[allow(dead_code)]
fn command_example() {
    let gnostr_sec = Command::new("gnostr")
        .arg("--sec")
        .arg("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
        .arg("-t")
        .arg("gnostr")
        .stdout(Stdio::piped())
        .output()
        .expect("url failed");
    let mut gnostr_event = String::from_utf8(gnostr_sec.stdout).unwrap();
    gnostr_event.pop();
    println!("{:?}", gnostr_event);

    let gnostr_post_event = Command::new("gnostr-post-event")
        .arg("--relay")
        .arg("wss://relay.damus.io")
        .arg(gnostr_event)
        .stdout(Stdio::piped())
        .output()
        .expect("picture failed");
    let post = String::from_utf8(gnostr_post_event.stdout).unwrap();
    println!("{}", post);
}


//reflog_simple
pub fn reflog_simple() -> Result<(), git2::Error> {

  let repo = match Repository::open(".") {
    Ok(repo) => repo,
    Err(e) => panic!("Error opening repository: {}", e),
};

let mut revwalk = repo.revwalk()?;

revwalk.push_head()?;
revwalk.set_sorting(git2::Sort::TIME)?;


for rev in revwalk {
    let commit = repo.find_commit(rev?)?;
    let message = commit.summary_bytes().unwrap_or_else(|| commit.message_bytes());
    println!("{:0>64}\n{}", commit.id(), String::from_utf8_lossy(message));
}

    Ok(())
}

fn format_commit(commit: &Commit, format_str: &str, show_date: bool, show_message: bool) -> String {
    // This function is not implemented in this example, but provides a placeholder for future enhancements
    // It would allow more flexible output formatting based on the provided format string and boolean flags.
    panic!("Formatting not implemented.");
}




fn main() {

    let args_vector: Vec<String> = env::args().collect();
    //println!("args_vector = {:?}", args_vector);
    //println!("args_vector.len() = {:?}", args_vector.len());


    //special case
    //execute ffi c code
    if args_vector.len() == 1 {

        //execute c if no other args
        //println!("args_vector = {:?}", args_vector);
        //println!("args_vector.len() = {:?}", args_vector.len());
        //println!("default HELP!");
        (assert_c! {
            #include <stdio.h>
            int main() {
                printf("Hello, Gnostr!");
                return 0;
            }
        })
        .success()
        .stdout("Hello, Gnostr!");
        //rust
        println!("Hello, Gnostr!");
        process::exit(0);
    }

    if args_vector.len() == 2 {

      //let _ = reflog_simple();

      //catch dump
      if args_vector[1] == "--dump" {
          let _ = get_self();
          process::exit(0);
      }
      //catch help
      if args_vector[1] == "-h" {
          println!("-h HELP!");
          process::exit(0);
      }
      if args_vector[1] == "--help" {
          println!("--help HELP!");
          process::exit(0);
      }
      //catch version
      if args_vector[1] == "-v" {
          println!("-v VERSION!");
          process::exit(0);
      }
      if args_vector[1] == "--version" {
          println!("--version VERSION!");
          process::exit(0);
      }
      //catch sec
      if args_vector[1] == "--sec" {
          println!("--sec CALLED!");
          process::exit(0);
      }
      //catch gen
      if args_vector[1] == "--gen" {
          //println!("--gen CALLED!");
          gen_keys();
          process::exit(0);
      }
      //catch genkey
      if args_vector[1] == "--genkey" {
          //println!("--genkey CALLED!");
          gen_keys();
          process::exit(0);
      }
      //catch genkeys
      if args_vector[1] == "--genkeys" {
          //println!("--genkeys CALLED!");
          gen_keys();
          process::exit(0);
      }
      //catch kind
      if args_vector[1] == "--kind" {
        println!("args_vector = {:?}", args_vector);
        println!("args_vector.len() = {:?}", args_vector.len());
        println!("--kind CALLED!");
        //catch missing int
        if args_vector.len() == 2 {
          println!("--kind HELP!");
          println!("{:?} CALLED!", args_vector[1]);
          println!("gnostr --kind <int>");
          process::exit(0);
        }
        if args_vector.len() > 2 {
          println!("gnostr --kind <int>");
          println!("{:?} {:?} CALLED!", args_vector[1],args_vector[2]);
          //process::exit(0);
        }
      }
    //catch legit
    if args_vector[1] == "--legit" {
        println!("--legit CALLED!");
        //gnostr_legit();
        process::exit(0);
    }
    //catch example
    if args_vector[1] == "--example" {
        println!("--example CALLED!");
        command_example();
        process::exit(0);
    }
    //catch commit
    if args_vector[1] == "--commit" {
        println!("--commit CALLED!");
        process::exit(0);
    }
    //catch reflog
    if args_vector[1] == "--reflog" {
      let _ = reflog_simple();
      process::exit(0);
    }

    //command_example();

    }// end if args_vector.len() == 1
    else { println!("default HELP!"); }


    if args_vector.len() > 1 {
    //catch kind
    if args_vector[1] == "--kind" {
        if args_vector.len() >= 2 {
          println!("--kind CALLED!");
          if args_vector[2] == "0" {
          set_kind();
          println!("--kind 0 CALLED!");
          }
          if args_vector[2] == "1" {
          set_kind();
          println!("--kind 1 CALLED!");
          }
        }
        else { println!("--kind CALLED but <int> not supported!"); }
        process::exit(0);


    }
    }

    (assert_c! {
        #include <stdio.h>

        int main() {
            printf("Hello, Gnostr!");

            return 0;
        }
    })
    .success()
    .stdout("Hello, Gnostr!");
    //rust
    println!("Hello, Gnostr!");

}