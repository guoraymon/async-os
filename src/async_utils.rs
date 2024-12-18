extern crate alloc;

use alloc::{boxed::Box, collections::vec_deque::VecDeque};
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};
use lazy_static::lazy_static;

use crate::{sync::UPSafeCell, timer::get_time_ms};

lazy_static! {
    pub static ref RUNTIME: UPSafeCell<Runtime> = unsafe { UPSafeCell::new(Runtime::new()) };
}

pub struct Runtime {
    tasks: VecDeque<Task>,
}

impl Runtime {
    fn new() -> Self {
        Runtime {
            tasks: VecDeque::new(),
        }
    }

    pub fn run(&mut self) {
        while let Some(mut task) = self.tasks.pop_front() {
            let waker = dummy_waker();
            let mut context = Context::from_waker(&waker);
            loop {
                match task.poll(&mut context) {
                    Poll::Ready(val) => break val,
                    Poll::Pending => {
                        self.tasks.push_back(task);
                        break;
                    }
                };
            }
        }
    }

    pub fn spawn(&mut self, future: impl Future<Output = ()> + Send + Sync + 'static) {
        self.tasks.push_back(Task::new(future))
    }
}

struct Task {
    future: Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + Send + Sync + 'static) -> Task {
        Task {
            future: Box::pin(future),
        }
    }

    fn poll(&mut self, cx: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(cx)
    }
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

fn dummy_raw_waker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }

    let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(0 as *const (), vtable)
}

pub async fn delay(ms: usize) {
    DelayFuture::new(ms).await;
}

struct DelayFuture {
    target_time: usize,
    waker: Option<Waker>,
}

impl DelayFuture {
    fn new(ms: usize) -> Self {
        DelayFuture {
            target_time: get_time_ms() + ms,
            waker: None,
        }
    }
}

impl Future for DelayFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if get_time_ms() >= self.target_time {
            Poll::Ready(())
        } else {
            self.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
