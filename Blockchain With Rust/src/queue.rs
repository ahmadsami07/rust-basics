use std::sync::mpsc;
use std::thread;

pub trait Task {
    type Output: Send;
    fn run(&self) -> Option<Self::Output>;
}

pub struct WorkQueue<TaskType: 'static + Task + Send> {
    send_tasks: Option<spmc::Sender<TaskType>>,
    recv_tasks: spmc::Receiver<TaskType>,
    //send_output: mpsc::Sender<TaskType::Output>, // not need in the struct: each worker will have its own clone.
    recv_output: mpsc::Receiver<TaskType::Output>,
    workers: Vec<thread::JoinHandle<()>>,
}

impl<TaskType: 'static + Task + Send> WorkQueue<TaskType> {
    pub fn new(n_workers: usize) -> WorkQueue<TaskType> {
        // create the channels; start the worker threads; record their JoinHandles
        let (sender, reciever) = spmc::channel();
        let (senderMpsc, recieverMpsc) = mpsc::channel();
        let mut joinHandles = Vec::new();
        for i in 0..n_workers {
            let recvSpmc = reciever.clone();
            let sndMpsc = senderMpsc.clone();
            joinHandles.push(thread::spawn(move || Self::run(recvSpmc, sndMpsc)));
        }

        return WorkQueue {
            send_tasks: Some(sender),
            recv_tasks: reciever,
            recv_output: recieverMpsc,
            workers: joinHandles,
        };
    }

    fn run(recv_tasks: spmc::Receiver<TaskType>, send_output: mpsc::Sender<TaskType::Output>) {
        // TODO: the main logic for a worker thread
        loop {
            let task_result = recv_tasks.recv();
            match task_result {
                Ok(task) => {
                    let taskOutput = task.run();
                    if taskOutput.is_some() {
                        send_output.send(taskOutput.unwrap());
                    }
                }
                Err(e) => {
                    break;
                }
            }
        }
    }

    pub fn enqueue(&mut self, t: TaskType) -> Result<(), spmc::SendError<TaskType>> {
        let sender = self.send_tasks.take();
        if sender.is_some() {
            let mut tempSnd = sender.unwrap();
            tempSnd.send(t);
            self.send_tasks = Some(tempSnd);
            return Ok(());
        }
        return Err(spmc::SendError(t));
    }

    // Helper methods that let you receive results in various ways
    pub fn iter(&mut self) -> mpsc::Iter<TaskType::Output> {
        self.recv_output.iter()
    }
    pub fn recv(&mut self) -> TaskType::Output {
        self.recv_output
            .recv()
            .expect("I have been shutdown incorrectly")
    }
    pub fn try_recv(&mut self) -> Result<TaskType::Output, mpsc::TryRecvError> {
        self.recv_output.try_recv()
    }
    pub fn recv_timeout(
        &self,
        timeout: std::time::Duration,
    ) -> Result<TaskType::Output, mpsc::RecvTimeoutError> {
        self.recv_output.recv_timeout(timeout)
    }
    pub fn shutdown(&mut self) {
        self.send_tasks = None;
        loop {
            let remaining_msg = self.recv_tasks.recv();
            if (remaining_msg.is_ok()) {
            } else {
                break;
            }
        }
        for handle in self.workers.drain(..) {
            handle.join();
        }
    }
}

impl<TaskType: 'static + Task + Send> Drop for WorkQueue<TaskType> {
    fn drop(&mut self) {
        // "Finalisation in destructors" pattern: https://rust-unofficial.github.io/patterns/idioms/dtor-finally.html
        match self.send_tasks {
            None => {} // already shut down
            Some(_) => self.shutdown(),
        }
    }
}
