use async_priority_channel as priority;
use miette::IntoDiagnostic;
use tokio::sync::{mpsc, watch};
use watchexec::{
    error::RuntimeError,
    event::{Event, Priority},
    fs::{worker, WatchedPath, WorkingData},
};

pub fn watcher<P, R>(
    paths: R,
) -> miette::Result<(
    watch::Sender<WorkingData>,
    mpsc::Receiver<RuntimeError>,
    priority::Receiver<Event, Priority>,
)>
where
    R: IntoIterator<Item = P>,
    P: Into<WatchedPath>,
{
    let (wd_s, wd_r) = watch::channel(WorkingData::default());
    let (ev_s, ev_r) = priority::bounded::<Event, Priority>(1024);
    let (er_s, er_r) = mpsc::channel(64);

    let mut wkd = WorkingData::default();
    wkd.pathset = paths.into_iter().map(|p| p.into()).collect();
    wd_s.send(wkd).into_diagnostic()?;

    tokio::spawn(async move {
        worker(wd_r, er_s, ev_s).await.into_diagnostic().unwrap();
    });

    Ok((wd_s, er_r, ev_r))
}
