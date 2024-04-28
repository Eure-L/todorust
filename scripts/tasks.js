function removeTaskItem(taskId, taskListId) {
    // Check if the task item is in the pending list
    const pendingList = document.getElementById(taskListId);
    const pendingTask = pendingList.querySelector(`#${taskId}`);
    if (pendingTask) {
        pendingList.removeChild(pendingTask);
    }
}

