use std::process::{Child, Command};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn run_supervisor(mpv_child: Arc<Mutex<Option<Child>>>) -> bool {
    let start = Instant::now();
    let mut playing = true;
    loop {
        let exited = {
            let mut g = mpv_child.lock().unwrap();
            match g.as_mut().unwrap().try_wait() {
                Ok(Some(_)) => true,
                Ok(None) => false,
                Err(_) => true,
            }
        };
        if exited {
            break;
        }
        if start.elapsed() > Duration::from_secs(25) && !playing {
            break;
        }
        thread::sleep(Duration::from_millis(50));
    }
    if playing {
        if let Some(c) = mpv_child.lock().unwrap().as_mut() {
            let _ = c.wait();
        }
    }
    playing
}

#[test]
fn next_episode_no_deadlock() {
    let child = Command::new("sleep")
        .arg("30")
        .spawn()
        .expect("spawn sleep");
    let mpv_child = Arc::new(Mutex::new(Some(child)));
    let (tx, rx) = mpsc::channel::<u64>();

    let mpv_child_sup = mpv_child.clone();
    let supervisor = thread::spawn(move || run_supervisor(mpv_child_sup));

    let mpv_child_w = mpv_child.clone();
    let tx_w = tx.clone();
    let worker = thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        if let Some(c) = mpv_child_w.lock().unwrap().as_mut() {
            let _ = c.kill();
        }
        let _ = tx_w.send(2u64);
    });

    let got = rx
        .recv_timeout(Duration::from_secs(5))
        .expect("DEADLOCK: worker child'i öldürüp mesajı yollayamadı");
    assert_eq!(got, 2, "sonraki bölüm mesajı yanlış");

    let played = supervisor.join().expect("supervisor panik");
    assert!(played, "supervisor child'in kapandığını algılamalı");
    worker.join().expect("worker panik");
}

#[test]
fn next_episode_prev_works() {
    let child = Command::new("sleep")
        .arg("30")
        .spawn()
        .expect("spawn sleep");
    let mpv_child = Arc::new(Mutex::new(Some(child)));
    let (tx, rx) = mpsc::channel::<u64>();

    let mpv_child_sup = mpv_child.clone();
    let supervisor = thread::spawn(move || run_supervisor(mpv_child_sup));

    let mpv_child_w = mpv_child.clone();
    let tx_w = tx.clone();
    let worker = thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        if let Some(c) = mpv_child_w.lock().unwrap().as_mut() {
            let _ = c.kill();
        }
        let _ = tx_w.send(1u64);
    });

    let got = rx
        .recv_timeout(Duration::from_secs(5))
        .expect("DEADLOCK: prev geçişi takıldı");
    assert_eq!(got, 1);
    assert!(supervisor.join().unwrap());
    worker.join().unwrap();
}

#[test]
fn dispatch_next_episode_invokes_play() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let ctx = glib::MainContext::default();
    let loop_ = glib::MainLoop::new(Some(&ctx), false);

    let play_called = Arc::new(AtomicUsize::new(0));
    let alive = Arc::new(std::sync::atomic::AtomicBool::new(true));

    let (next_tx, next_rx) = mpsc::channel::<u64>();
    let next_rx = Arc::new(Mutex::new(next_rx));

    let play_called_c = play_called.clone();
    let alive_c = alive.clone();
    let next_rx_c = next_rx.clone();
    glib::timeout_add_local(Duration::from_millis(150), move || {
        let msg = { next_rx_c.lock().unwrap().try_recv().ok() };
        if msg.is_some() {
            play_called_c.fetch_add(1, Ordering::SeqCst);
        }
        if alive_c.load(Ordering::SeqCst) {
            glib::ControlFlow::Continue
        } else {
            glib::ControlFlow::Break
        }
    });

    let next_tx_w = next_tx.clone();
    let alive_w = alive.clone();
    let loop_w = loop_.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        let _ = next_tx_w.send(2u64);
        thread::sleep(Duration::from_millis(100));
        alive_w.store(false, Ordering::SeqCst);
        thread::sleep(Duration::from_millis(50));
        loop_w.quit();
    });

    loop_.run();
    assert!(
        play_called.load(Ordering::SeqCst) >= 1,
        "play hiç çağrılmadı — dağıtım deseni bozuk"
    );
}
