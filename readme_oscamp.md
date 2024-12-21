首先，主要参考了 rCode 和 ArceOS，实现了基本的操作系统框架，主要实现堆内存分配，因为必须要使用 `Box` 和 `Pin`，还要实现打印功能用于测试，还需要实现时间获取，用于模拟异步延迟。

然后，实现了基本的异步任务调度框架，实现了 Runtime 和 Task，以及虚假的唤醒器，并实现了 DelayFuture，可以模拟异步延迟，测试整个异步任务调度的流程。

之后，提供了 `make build` 命令用于构建操作系统 和 `make run` 命令用于运行 qemu 测试，还提供了 `make` 命令同时运行两个命令。

最后，运行结果是看到任务调度先执行了 `task1` 输出 `start task 1`，然后 `task1` 遇到 `delay` 返回 `Pending`，任务切出开始执行 `task2`。 `task 2` 输出 `start task 2` 之后也遇到 `delay`返回 `Pending`，这时 CPU 在两个任务之间不断切换，直到 `task1` delay 首先 ready，输出 `end task 1`，然后任务调度切换到 task2 盲等至其 delay 返回 ready，输出 `end task 2` 之后，所有任务运行结束。