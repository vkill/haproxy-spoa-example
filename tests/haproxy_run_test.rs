use duct::cmd;
use smol::{Task, Timer};
use std::panic;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tempfile::tempdir;

mod test_utils;

#[test]
fn test() -> anyhow::Result<()> {
    smol::run(async {
        let listen_addr = test_utils::find_listen_addr().await;
        println!("listen_addr {}", listen_addr);

        let path = PathBuf::from("/opt/repos/haproxy-spoa-example");
        let dir = tempdir()?;

        let name = "haproxy-spoa-example".to_string();
        let handle = cmd!(
                "bash",
                "-c",
                format!("docker run --rm --name {} -v $(pwd)/haproxy_conf:/usr/local/etc/haproxy -v $(pwd)/haproxy_run:/var/run -e FE_BIND={} --network host haproxy:2.2-rc-alpine haproxy -f /usr/local/etc/haproxy/haproxy.cfg -d -V", &name, &listen_addr)
            )
            .dir(path)
            .unchecked().start()?;

        let handle = Arc::new(handle);
        let handle_thread = handle.clone();
        let handle_hook = handle.clone();
        let name_hook = name.clone();

        panic::set_hook(Box::new(move |_| {
            match clean(&handle_hook, &name_hook) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
            ()
        }));

        let haproxy = Task::<anyhow::Result<()>>::local(async move {
            thread::spawn(move || {
                let output = handle_thread.wait();
                println!("haproxy output {:?}", output);
            });

            Ok(())
        });

        let client = Task::<anyhow::Result<()>>::spawn(async move {
            Timer::after(Duration::from_secs(1)).await;

            Ok(())
        });

        client.await?;
        haproxy.cancel().await;

        let _ = panic::take_hook();
        clean(&handle, &name)?;

        Ok(())
    })
}

fn clean(handle: &Arc<duct::Handle>, name: &str) -> anyhow::Result<()> {
    handle.kill()?;

    let handle = cmd!("bash", "-c", format!("docker rm {} -f", name)).start()?;
    let output = handle.wait()?;
    println!("haproxy clean output {:?}", output);

    Ok(())
}
