import * as React from "react";
import Card from "@mui/material/Card";
import CardContent from "@mui/material/CardContent";
import Button from "@mui/material/Button";
import Typography from "@mui/material/Typography";
import AutoAwesomeRoundedIcon from "@mui/icons-material/AutoAwesomeRounded";
import Box from "@mui/material/Box";
import LinearProgress from "@mui/material/LinearProgress";
import { useQuery } from "@tanstack/react-query";

export default function CardAlert() {
  const { data: authUser } = useQuery({ queryKey: ["authUser"] });

  React.useEffect(() =>{
    console.log(authUser);

  },[authUser])
  

  const [usageCount, setUsageCount] = React.useState(0);
  
  // Set maxUsage based on authUser's existence
  const maxUsage = authUser ? authUser.usage?.max_tries || 10 : 5;
  
  // If authUser is present, set triesUsed from their data, otherwise it is 0
  const triesUsed = authUser ? authUser.usage?.tries_used || 0 : usageCount;

  // On click, update the local usageCount and authUser's tries_used if present
  const handleClick = () => {
    if (authUser) {
      // Update the tries_used in authUser data
      if (triesUsed < maxUsage) {
        setUsageCount((prevCount) => prevCount + 1);
        authUser.usage.tries_used += 1; // This modifies the client-side state only
        console.log(authUser.usage.tries_used)
      }
    } else {
      // For users without authUser, update local usageCount
      if (usageCount < maxUsage) {
        setUsageCount((prevCount) => prevCount + 1);
      }
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
          {triesUsed === maxUsage
            ? "You've reached the limit."
            : `You have ${maxUsage - triesUsed} free calls left.`}
        </Typography>
        <Box sx={{ width: "100%", mb: 1 }}>
          <LinearProgress
            variant="determinate"
            value={(triesUsed / maxUsage) * 100}
            color="secondary"
          />
        </Box>
        <Button
          variant="contained"
          size="small"
          fullWidth
          onClick={handleClick}
          disabled={triesUsed === maxUsage}
        >
          {triesUsed === maxUsage ? "Upgrade your plan" : "Use a free call"}
        </Button>
      </CardContent>
    </Card>
  );
}






// import * as React from "react";
// import Card from "@mui/material/Card";
// import CardContent from "@mui/material/CardContent";
// import Button from "@mui/material/Button";
// import Typography from "@mui/material/Typography";
// import AutoAwesomeRoundedIcon from "@mui/icons-material/AutoAwesomeRounded";
// import Box from "@mui/material/Box";
// import LinearProgress from "@mui/material/LinearProgress";
// import { useQuery } from "@tanstack/react-query";

// export default function CardAlert() {
//   const { data: authUser } = useQuery({ queryKey: ["authUser"] });
//   React.useEffect(() => {
//     console.log(authUser);
//     // usage: { tries_used: 0, max_tries: 10 },
//   }, [authUser]);

//   const [usageCount, setUsageCount] = React.useState(0);
//   const maxUsage = authUser?.usage?.max_tries;

//   const handleClick = () => {
//     if (usageCount < maxUsage) {
//       setUsageCount((prevCount) => prevCount + 1);
//     }
//   };

//   return (
//     <Card
//       variant="outlined"
//       sx={{
//         width: "100%",
//         bgcolor: "background.paper",
//         boxShadow: "none",
//         border: "none",
//       }}
//     >
//       <CardContent sx={{ p: 1, "&:last-child": { pb: 1 } }}>
//         <Box sx={{ display: "flex", alignItems: "center", mb: 0.5 }}>
//           <AutoAwesomeRoundedIcon fontSize="small" sx={{ mr: 1 }} />
//           <Typography variant="subtitle2" sx={{ fontWeight: 600 }}>
//             Usage Tracker
//           </Typography>
//         </Box>
//         <Typography
//           variant="caption"
//           sx={{ display: "block", mb: 1, color: "text.secondary" }}
//         >
//           {usageCount === maxUsage
//             ? "You've reached the limit."
//             : `You have ${maxUsage - usageCount} free calls left.`}
//         </Typography>
//         <Box sx={{ width: "100%", mb: 1 }}>
//           <LinearProgress
//             variant="determinate"
//             value={(usageCount / maxUsage) * 100}
//             color="secondary"
//           />
//         </Box>
//         <Button
//           variant="contained"
//           size="small"
//           fullWidth
//           onClick={handleClick}
//           disabled={usageCount === maxUsage}
//         >
//           {usageCount === maxUsage ? "Upgrade your plan" : "Use a free call"}
//         </Button>
//       </CardContent>
//     </Card>
//   );
// }
