import React from 'react';
import { 
  Box, 
  Typography, 
  Container, 
  Paper, 
  Button, 
  List, 
  ListItem, 
  ListItemIcon, 
  ListItemText,
  Divider
} from '@mui/material';
import { ArrowForward } from '@mui/icons-material';
import { styled, keyframes } from '@mui/system';

const gradientAnimation = keyframes`
  0% { background-position: 0% 50%; }
  50% { background-position: 100% 50%; }
  100% { background-position: 0% 50%; }
`;

const ProfessionalBackground = styled(Box)({
  background: 'linear-gradient(-45deg, #f0f0f0, #ffffff, #e0e0e0, #f5f5f5)',
  backgroundSize: '400% 400%',
  animation: `${gradientAnimation} 15s ease infinite`,
  minHeight: '100vh',
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
});

const StyledPaper = styled(Paper)(({ theme }) => ({
  padding: theme.spacing(6),
  backgroundColor: 'rgba(255, 255, 255, 0.9)',
  borderRadius: '16px',
  boxShadow: '0 10px 30px rgba(0, 0, 0, 0.1)',
  backdropFilter: 'blur(10px)',
  [theme.breakpoints.up('sm')]: {
    padding: theme.spacing(8),
  },
}));

const crossAnimation = keyframes`
  0% { transform: scale(0.5); opacity: 0; }
  50% { transform: scale(1.2); }
  100% { transform: scale(1); opacity: 1; }
`;

const AnimatedCross = styled('svg')({
  animation: `${crossAnimation} 0.6s cubic-bezier(0.175, 0.885, 0.32, 1.275) forwards`,
  opacity: 0,
  width: '80px',
  height: '80px',
});

const FailurePage = () => {
  return (
    <ProfessionalBackground>
      <Container maxWidth="sm">
        <StyledPaper elevation={24} className="text-center">
          <AnimatedCross viewBox="0 0 100 100" className="mx-auto mb-6">
            <circle cx="50" cy="50" r="45" fill="#d32f2f" />
            <path d="M30 30 L70 70 M70 30 L30 70" stroke="white" strokeWidth="8" strokeLinecap="round" />
          </AnimatedCross>
          <Typography variant="h3" component="h1" gutterBottom className="font-bold text-gray-900 mb-4">
            Payment Failed
          </Typography>
          <Typography variant="subtitle1" className="text-gray-600 mb-8">
            We apologize, but there was an issue processing your payment. Please review the details below.
          </Typography>
          
          <Divider className="my-8" />
          
          <Box mt={4}>
            <Typography variant="h5" component="h2" gutterBottom className="text-gray-800 font-semibold mb-4">
              Possible Reasons
            </Typography>
            <List>
              {[
                'Insufficient funds in the account',
                'Incorrect card information provided',
                'Temporary issue with the payment gateway',
                'Card has expired or been cancelled'
              ].map((reason, index) => (
                <ListItem key={index} disablePadding className="mb-2">
                  <ListItemIcon>
                    <ArrowForward className="text-gray-600" />
                  </ListItemIcon>
                  <ListItemText primary={reason} className="text-gray-600" />
                </ListItem>
              ))}
            </List>
          </Box>
          
          <Box mt={6} display="flex" justifyContent="center" gap={3}>
            <Button 
              variant="contained" 
              color="primary" 
              href="/retry-payment"
              size="large"
              className="bg-gray-800 hover:bg-gray-700 text-white px-6 py-3 rounded-full transition-all duration-300 ease-in-out transform hover:scale-105"
            >
              Retry Payment
            </Button>
            <Button 
              variant="outlined" 
              color="inherit" 
              href="/"
              size="large"
              className="text-gray-800 border-gray-300 hover:bg-gray-100 px-6 py-3 rounded-full transition-all duration-300 ease-in-out transform hover:scale-105"
            >
              Return to Homepage
            </Button>
          </Box>
        </StyledPaper>
      </Container>
    </ProfessionalBackground>
  );
};

export default FailurePage;