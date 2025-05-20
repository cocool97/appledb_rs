import React, { useEffect, useState } from "react";
import { Box, LinearProgress, List, ListItem, Typography } from "@mui/material";
import { GET_RUNNING_TASKS } from "../Constants";
import { TaskProgress } from "../types/task_progress";

interface LinearProgressWithLabelProps {
  value: number;
}

function LinearProgressWithLabel({ value }: LinearProgressWithLabelProps) {
  return (
    <Box sx={{ display: "flex", alignItems: "center", width: "100%" }}>
      <Box sx={{ width: "100%", mr: 1 }}>
        <LinearProgress variant="determinate" value={value} />
      </Box>
      <Box sx={{ minWidth: 35 }}>
        <Typography variant="body2" sx={{ color: "white", fontWeight: "bold" }}>
          {`${Math.round(value)}%`}
        </Typography>
      </Box>
    </Box>
  );
}

interface TasksListProps {
  tasks: Record<string, TaskProgress>;
}

const TasksList = ({ tasks }: TasksListProps) => (
  <List sx={{ width: "100%" }}>
    {Object.entries(tasks).map(
      ([task_uuid, { task_type, start_time, done, total }]) => {
        const progress = total > 0 ? (done / total) * 100 : 0;
        const readableDate = new Date(start_time * 1000).toLocaleString();

        return (
          <ListItem
            key={task_uuid}
            sx={{
              display: "flex",
              flexDirection: "column",
              alignItems: "flex-start",
              mb: 3,
              backgroundColor: "#1e1e1e",
              p: 2,
              borderRadius: 2,
              boxShadow: 2,
            }}
          >
            <Typography
              variant="subtitle1"
              sx={{ color: "white", fontWeight: "bold" }}
            >
              {task_type}
            </Typography>
            <Typography variant="caption" sx={{ color: "gray" }}>
              UUID: {task_uuid}
            </Typography>
            <Typography variant="caption" sx={{ color: "gray", mb: 1 }}>
              Start: {readableDate}
            </Typography>
            <LinearProgressWithLabel value={progress} />
          </ListItem>
        );
      },
    )}
  </List>
);

const Tasks = () => {
  const [tasks, setTasks] = useState<Record<string, TaskProgress>>({});
  const TASKS_REFRESH_INTERVAL_MS = 2000;

  useEffect(() => {
    const fetchTasks = () => {
      fetch(`${GET_RUNNING_TASKS}`)
        .then((response) => response.json())
        .then((data: Record<string, TaskProgress>) => {
          const sortedTasks = Object.entries(data)
            .sort(([, taskA], [, taskB]) => taskA.start_time - taskB.start_time)
            .reduce<Record<string, TaskProgress>>((acc, [uuid, task]) => {
              acc[uuid] = task;
              return acc;
            }, {});
          setTasks(sortedTasks);
        })
        .catch((error) => console.error("Error fetching tasks:", error));
    };

    fetchTasks();

    const interval = setInterval(fetchTasks, TASKS_REFRESH_INTERVAL_MS);

    return () => clearInterval(interval);
  }, []);

  return (
    <Box sx={{ p: 2 }}>
      <Typography variant="h6" sx={{ mb: 2, fontWeight: "bold" }}>
        Running tasks ({Object.keys(tasks).length})
      </Typography>
      <TasksList tasks={tasks} />
    </Box>
  );
};

export default Tasks;
