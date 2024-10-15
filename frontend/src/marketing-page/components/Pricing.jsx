import * as React from "react";
import Box from "@mui/material/Box";
import Button from "@mui/material/Button";
import Card from "@mui/material/Card";
import Chip from "@mui/material/Chip";
import CardActions from "@mui/material/CardActions";
import CardContent from "@mui/material/CardContent";
import Container from "@mui/material/Container";
import Divider from "@mui/material/Divider";
import Grid from "@mui/material/Grid2";
import Typography from "@mui/material/Typography";
import Zoom from "@mui/material/Zoom";
import { styled } from "@mui/material/styles";
import ToggleButton from "@mui/material/ToggleButton";
import ToggleButtonGroup from "@mui/material/ToggleButtonGroup";

import AutoAwesomeIcon from "@mui/icons-material/AutoAwesome";
import CheckCircleRoundedIcon from "@mui/icons-material/CheckCircleRounded";
import ArrowUpwardIcon from "@mui/icons-material/ArrowUpward";

const AnimatedCard = styled(Card)(({ theme }) => ({
  transition: "transform 0.3s ease-in-out, box-shadow 0.3s ease-in-out",
  "&:hover": {
    transform: "translateY(-10px)",
    boxShadow: theme.shadows[10],
  },
}));

const CustomToggleButton = styled(ToggleButton)(({ theme }) => ({
  borderRadius: "20px",
  border: "none",
  textTransform: "none",
  padding: "6px 16px",
  color: theme.palette.primary.main,
  fontWeight: 600,
  "&.Mui-selected": {
    backgroundColor: theme.palette.primary.main,
    color: "#fff",
    boxShadow: "0 4px 10px rgba(0, 0, 0, 0.2)",
  },
}));

const BillingToggle = styled(ToggleButtonGroup)(({ theme }) => ({
  backgroundColor: "transparent",
  border: `2px solid ${theme.palette.divider}`,
  borderRadius: "30px",
  padding: "4px",
  "& .MuiToggleButton-root": {
    border: "none",
  },
}));

const tiers = [
  {
    title: "Basic",
    monthlyPrice: "0.00",
    yearlyPrice: "00.00",
    description: [
      "AI Translation: 1000 words/month",
      "AI Translation: Text only",
      "AI Image Replacer: 10 images/month",
      "Email support",
    ],
    buttonText: "Start Free Trial",
    buttonVariant: "outlined",
    buttonColor: "primary",
  },
  {
    plan_id_monthly: "price_1Q9kyZRtqMxXmkr408G1L9Kg",
    plan_id_yearly: "price_1Q9l0eRtqMxXmkr4f7i32obw",
    title: "Pro",
    subheader: "Most Popular",
    monthlyPrice: "29.99",
    yearlyPrice: "299.99",
    description: [
      "Everything in Basic, plus:",
      "AI Translation: 5000 words/month",
      "AI Translation: File upload",
      "AI Image Replacer: 50 images/month",
      "Chat with AI: 100 messages/month",
      "Priority email support",
    ],
    buttonText: "Get Started",
    buttonVariant: "contained",
    buttonColor: "secondary",
  },
  {
    plan_id_monthly: "price_1Q9kzfRtqMxXmkr4N2jVAj0d",
    plan_id_yearly: "price_1Q9l12RtqMxXmkr4orIpbeNL",
    title: "Enterprise",
    monthlyPrice: "99.99",
    yearlyPrice: "999.99",
    description: [
      "Everything in Pro, plus:",
      "AI Translation: Unlimited words",
      "AI Translation: Chat integration",
      "AI Image Replacer: Unlimited images",
      "Chat with AI: Unlimited messages",
      "Dedicated account manager",
      "Phone & email support",
    ],
    buttonText: "Contact Sales",
    buttonVariant: "outlined",
    buttonColor: "primary",
  },
];

export default function Pricing() {
  const [hoveredTier, setHoveredTier] = React.useState(null);
  const [billingCycle, setBillingCycle] = React.useState("monthly");

  const handleBillingCycleChange = (event, newBillingCycle) => {
    if (newBillingCycle !== null) {
      setBillingCycle(newBillingCycle);
    }
  };

  return (
    <Container
      id="pricing"
      sx={{
        pt: { xs: 4, sm: 12 },
        pb: { xs: 8, sm: 16 },
        position: "relative",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        gap: { xs: 3, sm: 6 },
      }}
    >
      <Box
        sx={{
          width: { sm: "100%", md: "60%" },
          textAlign: { sm: "left", md: "center" },
        }}
      >
        <Typography
          component="h2"
          variant="h4"
          gutterBottom
          sx={{ color: "text.primary" }}
        >
          Pricing Plans
        </Typography>
        <Typography variant="body1" sx={{ color: "text.secondary", mb: 3 }}>
          Choose the perfect plan for your AI-powered needs
        </Typography>

        {/* Billing Cycle Toggle Button */}
        <BillingToggle
          value={billingCycle}
          exclusive
          onChange={handleBillingCycleChange}
          aria-label="billing cycle"
          sx={{
            mb: 4,
            display: "flex",
            justifyContent: "center",
            maxWidth: "180px", // Limits the width to fit around the two buttons
            mx: "auto", // Centers the BillingToggle horizontally
            padding: "4px", // Adjusts padding to make it more compact
          }}
        >
          <CustomToggleButton value="monthly" aria-label="monthly billing">
            Monthly
          </CustomToggleButton>
          <CustomToggleButton value="yearly" aria-label="yearly billing">
            Yearly
          </CustomToggleButton>
        </BillingToggle>
      </Box>

      <Grid
        container
        spacing={3}
        sx={{ alignItems: "center", justifyContent: "center", width: "100%" }}
      >
        {tiers.map((tier, index) => (
          <Grid
            xs={12}
            sm={tier.title === "Enterprise" ? 12 : 6}
            md={4}
            key={tier.title}
          >
            <Zoom in={true} style={{ transitionDelay: `${index * 100}ms` }}>
              <AnimatedCard
                onMouseEnter={() => setHoveredTier(tier.title)}
                onMouseLeave={() => setHoveredTier(null)}
                sx={[
                  {
                    p: 2,
                    display: "flex",
                    flexDirection: "column",
                    gap: 4,
                    height: "100%",
                  },
                  tier.title === "Pro" && {
                    border: "2px solid",
                    borderColor: "secondary.main",
                  },
                ]}
              >
                <CardContent>
                  <Box
                    sx={{
                      mb: 1,
                      display: "flex",
                      justifyContent: "space-between",
                      alignItems: "center",
                      gap: 2,
                    }}
                  >
                    <Typography component="h3" variant="h6">
                      {tier.title}
                    </Typography>
                    {tier.title === "Pro" && (
                      <Chip
                        icon={<AutoAwesomeIcon />}
                        label={tier.subheader}
                        color="secondary"
                      />
                    )}
                  </Box>
                  <Box
                    sx={{
                      display: "flex",
                      alignItems: "baseline",
                      mb: 2,
                    }}
                  >
                    <Typography component="h3" variant="h3">
                      $
                      {billingCycle === "monthly"
                        ? tier.monthlyPrice
                        : tier.yearlyPrice}
                    </Typography>
                    <Typography component="span" variant="h6">
                      &nbsp; /{billingCycle === "monthly" ? "month" : "year"}
                    </Typography>
                  </Box>
                  {billingCycle === "yearly" && (
                    <Typography
                      variant="body2"
                      color="success.main"
                      sx={{ mb: 2 }}
                    >
                      Save{" "}
                      {!isNaN(
                        Math.round(
                          (1 - tier.yearlyPrice / (tier.monthlyPrice * 12)) *
                            100
                        )
                      ) &&
                      Math.round(
                        (1 - tier.yearlyPrice / (tier.monthlyPrice * 12)) * 100
                      ) !== 0
                        ? Math.round(
                            (1 - tier.yearlyPrice / (tier.monthlyPrice * 12)) *
                              100
                          ) + "%"
                        : "0%"}
                    </Typography>
                  )}
                  <Divider
                    sx={{ my: 2, opacity: 0.8, borderColor: "divider" }}
                  />
                  {tier.description.map((line, lineIndex) => (
                    <Box
                      key={line}
                      sx={{
                        py: 1,
                        display: "flex",
                        gap: 1.5,
                        alignItems: "center",
                        opacity:
                          hoveredTier === tier.title || lineIndex === 0
                            ? 1
                            : 0.7,
                        transition: "opacity 0.3s ease-in-out",
                      }}
                    >
                      <CheckCircleRoundedIcon
                        sx={{
                          width: 20,
                          color: "primary.main",
                        }}
                      />
                      <Typography variant="body2" component="span">
                        {line}
                      </Typography>
                    </Box>
                  ))}
                </CardContent>
                <CardActions sx={{ mt: "auto" }}>
                  <Button
                    fullWidth
                    variant={tier.buttonVariant}
                    color={tier.buttonColor}
                    endIcon={<ArrowUpwardIcon />}
                  >
                    {tier.buttonText}
                  </Button>
                </CardActions>
              </AnimatedCard>
            </Zoom>
          </Grid>
        ))}
      </Grid>
    </Container>
  );
}

// import * as React from 'react';
// import Box from '@mui/material/Box';
// import Button from '@mui/material/Button';
// import Card from '@mui/material/Card';
// import Chip from '@mui/material/Chip';
// import CardActions from '@mui/material/CardActions';
// import CardContent from '@mui/material/CardContent';
// import Container from '@mui/material/Container';
// import Divider from '@mui/material/Divider';
// import Grid from '@mui/material/Grid2';
// import Typography from '@mui/material/Typography';

// import AutoAwesomeIcon from '@mui/icons-material/AutoAwesome';
// import CheckCircleRoundedIcon from '@mui/icons-material/CheckCircleRounded';

// const tiers = [
//   {
//     title: 'Free',
//     price: '0',
//     description: [
//       '10 users included',
//       '2 GB of storage',
//       'Help center access',
//       'Email support',
//     ],
//     buttonText: 'Sign up for free',
//     buttonVariant: 'outlined',
//     buttonColor: 'primary',
//   },
//   {
//     title: 'Professional',
//     subheader: 'Recommended',
//     price: '15',
//     description: [
//       '20 users included',
//       '10 GB of storage',
//       'Help center access',
//       'Priority email support',
//       'Dedicated team',
//       'Best deals',
//     ],
//     buttonText: 'Start now',
//     buttonVariant: 'contained',
//     buttonColor: 'secondary',
//   },
//   {
//     title: 'Enterprise',
//     price: '30',
//     description: [
//       '50 users included',
//       '30 GB of storage',
//       'Help center access',
//       'Phone & email support',
//     ],
//     buttonText: 'Contact us',
//     buttonVariant: 'outlined',
//     buttonColor: 'primary',
//   },
// ];

// export default function Pricing() {
//   return (
//     <Container
//       id="pricing"
//       sx={{
//         pt: { xs: 4, sm: 12 },
//         pb: { xs: 8, sm: 16 },
//         position: 'relative',
//         display: 'flex',
//         flexDirection: 'column',
//         alignItems: 'center',
//         gap: { xs: 3, sm: 6 },
//       }}
//     >
//       <Box
//         sx={{
//           width: { sm: '100%', md: '60%' },
//           textAlign: { sm: 'left', md: 'center' },
//         }}
//       >
//         <Typography
//           component="h2"
//           variant="h4"
//           gutterBottom
//           sx={{ color: 'text.primary' }}
//         >
//           Pricing
//         </Typography>
//         <Typography variant="body1" sx={{ color: 'text.secondary' }}>
//           Quickly build an effective pricing table for your potential customers with
//           this layout. <br />
//           It&apos;s built with default Material UI components with little
//           customization.
//         </Typography>
//       </Box>
//       <Grid
//         container
//         spacing={3}
//         sx={{ alignItems: 'center', justifyContent: 'center', width: '100%' }}
//       >
//         {tiers.map((tier) => (
//           <Grid
//             size={{ xs: 12, sm: tier.title === 'Enterprise' ? 12 : 6, md: 4 }}
//             key={tier.title}
//           >
//             <Card
//               sx={[
//                 {
//                   p: 2,
//                   display: 'flex',
//                   flexDirection: 'column',
//                   gap: 4,
//                 },
//                 tier.title === 'Professional' &&
//                   ((theme) => ({
//                     border: 'none',
//                     background:
//                       'radial-gradient(circle at 50% 0%, hsl(220, 20%, 35%), hsl(220, 30%, 6%))',
//                     boxShadow: `0 8px 12px hsla(220, 20%, 42%, 0.2)`,
//                     ...theme.applyStyles('dark', {
//                       background:
//                         'radial-gradient(circle at 50% 0%, hsl(220, 20%, 20%), hsl(220, 30%, 16%))',
//                       boxShadow: `0 8px 12px hsla(0, 0%, 0%, 0.8)`,
//                     }),
//                   })),
//               ]}
//             >
//               <CardContent>
//                 <Box
//                   sx={[
//                     {
//                       mb: 1,
//                       display: 'flex',
//                       justifyContent: 'space-between',
//                       alignItems: 'center',
//                       gap: 2,
//                     },
//                     tier.title === 'Professional'
//                       ? { color: 'grey.100' }
//                       : { color: '' },
//                   ]}
//                 >
//                   <Typography component="h3" variant="h6">
//                     {tier.title}
//                   </Typography>
//                   {tier.title === 'Professional' && (
//                     <Chip icon={<AutoAwesomeIcon />} label={tier.subheader} />
//                   )}
//                 </Box>
//                 <Box
//                   sx={[
//                     {
//                       display: 'flex',
//                       alignItems: 'baseline',
//                     },
//                     tier.title === 'Professional'
//                       ? { color: 'grey.50' }
//                       : { color: null },
//                   ]}
//                 >
//                   <Typography component="h3" variant="h2">
//                     ${tier.price}
//                   </Typography>
//                   <Typography component="h3" variant="h6">
//                     &nbsp; per month
//                   </Typography>
//                 </Box>
//                 <Divider sx={{ my: 2, opacity: 0.8, borderColor: 'divider' }} />
//                 {tier.description.map((line) => (
//                   <Box
//                     key={line}
//                     sx={{ py: 1, display: 'flex', gap: 1.5, alignItems: 'center' }}
//                   >
//                     <CheckCircleRoundedIcon
//                       sx={[
//                         {
//                           width: 20,
//                         },
//                         tier.title === 'Professional'
//                           ? { color: 'primary.light' }
//                           : { color: 'primary.main' },
//                       ]}
//                     />
//                     <Typography
//                       variant="subtitle2"
//                       component={'span'}
//                       sx={[
//                         tier.title === 'Professional'
//                           ? { color: 'grey.50' }
//                           : { color: null },
//                       ]}
//                     >
//                       {line}
//                     </Typography>
//                   </Box>
//                 ))}
//               </CardContent>
//               <CardActions>
//                 <Button
//                   fullWidth
//                   variant={tier.buttonVariant}
//                   color={tier.buttonColor}
//                 >
//                   {tier.buttonText}
//                 </Button>
//               </CardActions>
//             </Card>
//           </Grid>
//         ))}
//       </Grid>
//     </Container>
//   );
// }
