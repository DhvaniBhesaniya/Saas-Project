import * as React from "react";
import Card from "@mui/material/Card";
import CardContent from "@mui/material/CardContent";
import Button from "@mui/material/Button";
import Typography from "@mui/material/Typography";
import AutoAwesomeRoundedIcon from "@mui/icons-material/AutoAwesomeRounded";
import Box from "@mui/material/Box";
import LinearProgress from "@mui/material/LinearProgress";

export default function CardAlert() {
  const [usageCount, setUsageCount] = React.useState(0);
  const maxUsage = 5;

  const handleClick = () => {
    if (usageCount < maxUsage) {
      setUsageCount(prevCount => prevCount + 1);
    }
  };

  return (
    <Card
      variant="outlined"
      sx={{
        width: "100%",
        bgcolor: "background.paper",
        boxShadow: "none",
        border: "none",
      }}
    >
      <CardContent sx={{ p: 1, "&:last-child": { pb: 1 } }}>
        <Box sx={{ display: "flex", alignItems: "center", mb: 0.5 }}>
          <AutoAwesomeRoundedIcon fontSize="small" sx={{ mr: 1 }} />
          <Typography variant="subtitle2" sx={{ fontWeight: 600 }}>
            Usage Tracker
          </Typography>
        </Box>
        <Typography
          variant="caption"
          sx={{ display: "block", mb: 1, color: "text.secondary" }}
        >
          {usageCount === maxUsage 
            ? "You've reached the limit." 
            : `You have ${maxUsage - usageCount} free calls left.`}
        </Typography>
        <Box sx={{ width: '100%', mb: 1 }}>
          <LinearProgress 
            variant="determinate" 
            value={(usageCount / maxUsage) * 100}
            color="secondary" 
          />
        </Box>
        <Button 
          variant="contained" 
          size="small" 
          fullWidth
          onClick={handleClick}
          disabled={usageCount === maxUsage}
        >
          {usageCount === maxUsage ? "Upgrade your plan" : "Use a free call"}
        </Button>
      </CardContent>
    </Card>
  );
}