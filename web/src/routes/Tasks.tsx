import React, { useState, useEffect } from 'react';
import './HomePage.css';
import { Box, LinearProgress, List, ListItem, ListItemAvatar, Typography } from '@mui/material';
import { GET_RUNNING_TASKS } from '../Constants';

function LinearProgressWithLabel(props) {
    return (
        <Box sx={{ display: 'flex', alignItems: 'center', width: "100%" }}>
            <Box sx={{ width: '100%', mr: 1 }}>
                <LinearProgress variant="determinate" {...props} />
            </Box>
            <Box sx={{ minWidth: 35 }}>
                <Typography variant="body2" sx={{ color: 'white', fontWeight: "bold" }}>
                    {`${Math.round(props.value)}%`}
                </Typography>
            </Box>
        </Box>
    );
}

const TasksList = (props) => {
    return (
        <List sx={{ width: '100%' }}>
            {Object.entries(props.tasks).map(([task_uuid, { done, total }]) => (
                <ListItem key={task_uuid}>
                    <ListItemAvatar sx={{marginRight: "1rem"}}>
                        <div>{task_uuid}</div>
                    </ListItemAvatar>
                    <LinearProgressWithLabel value={done / total * 100} />
                </ListItem>
            ))}
        </List>
    );
};

const Tasks = () => {
    const [tasks, setTasks] = useState({});

    useEffect(() => {
        const interval = setInterval(() => {
            fetch(`${GET_RUNNING_TASKS}`)
                .then((response) => response.json())
                .then((data) => setTasks(data))
                .catch((error) => console.log(error));
        }, 2000);

        return () => clearInterval(interval);
    }, []);

    return (
        <div>
            <Typography variant="h6">Running tasks ({Object.keys(tasks).length})</Typography>
            <TasksList tasks={tasks} />
        </div>
    );
};

export default Tasks;
