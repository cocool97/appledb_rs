import { Box, CircularProgress, Typography } from "@mui/material";
import React, { JSX, useEffect, useState } from "react";
import { API_URL } from "../Constants";
import Card from "../components/Card";
import { MdPhoneIphone } from "react-icons/md";
import { ServerStats } from "../types/server_stats";

const labelMap: Record<keyof ServerStats, string> = {
  known_devices: "Devices",
  known_operating_system_versions: "OS versions",
  known_executables: "Executables",
  known_entitlements: "Entitlements",
  known_frameworks: "Frameworks",
};

const iconMap: Record<keyof ServerStats, JSX.Element> = {
  known_devices: <MdPhoneIphone />,
  known_operating_system_versions: <MdPhoneIphone />,
  known_executables: <MdPhoneIphone />,
  known_entitlements: <MdPhoneIphone />,
  known_frameworks: <MdPhoneIphone />,
};

const Home: React.FC = () => {
  const [stats, setStats] = useState<ServerStats | null>(null);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(true);

  useEffect(() => {
    fetch(`${API_URL}/stats`)
      .then((res) => res.json())
      .then((data: ServerStats) => {
        setStats(data);
        setLoading(false);
      })
      .catch((err) => {
        setErrorMessage(err.message);
        setLoading(false);
      });
  }, []);

  if (loading) {
    return (
      <Box sx={{ textAlign: "center" }}>
        <CircularProgress />
        <Typography color="white" sx={{ mt: 2 }}>
          Loading stats...
        </Typography>
      </Box>
    );
  }

  if (!stats) {
    return (
      <Typography color="white" textAlign="center">
        Failed to load stats: {errorMessage}
      </Typography>
    );
  }

  const renderStats = () => {
    if (Object.entries(stats).length === 0) {
      return (
        <div
          style={{
            width: "inherit",
            height: "inherit",
            textAlign: "center",
            fontWeight: "bold",
            fontSize: "2.5rem",
          }}
        >
          No models found
        </div>
      );
    }

    return Object.entries(stats).map(([key, value]) => {
      const typedKey = key as keyof ServerStats;

      return (
        <Card
          key={key}
          icon={iconMap[typedKey] || <MdPhoneIphone />}
          main={labelMap[typedKey] || key}
          secondary={value}
        />
      );
    });
  };

  return (
    <Box
      sx={{ display: "flex", flexWrap: "wrap", justifyContent: "space-around" }}
    >
      {renderStats()}
    </Box>
  );
};

export default Home;
