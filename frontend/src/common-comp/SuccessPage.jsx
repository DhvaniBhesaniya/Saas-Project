import React from 'react';
import { 
  Box, 
  Typography, 
  Container, 
  Paper, 
  Button, 
  List, 
  ListItem, 
  ListItemText,
  Divider
} from '@mui/material';
import { CheckCircle } from '@mui/icons-material';
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

const checkAnimation = keyframes`
  0% { transform: scale(0.5); opacity: 0; }
  50% { transform: scale(1.2); }
  100% { transform: scale(1); opacity: 1; }
`;

const AnimatedCheckCircle = styled(CheckCircle)({
  animation: `${checkAnimation} 0.6s cubic-bezier(0.175, 0.885, 0.32, 1.275) forwards`,
  opacity: 0,
  color: '#2e7d32',
  fontSize: 80,
});

const SuccessPage = () => {
  return (
    <ProfessionalBackground>
      <Container maxWidth="sm">
        <StyledPaper elevation={24} className="text-center">
          <AnimatedCheckCircle className="mx-auto mb-6" />
          <Typography variant="h3" component="h1" gutterBottom className="font-bold text-gray-900 mb-4">
            Payment Successful
          </Typography>
          <Typography variant="subtitle1" className="text-gray-600 mb-8">
            Thank you for your purchase. Your transaction has been completed successfully.
          </Typography>
          
          <Divider className="my-8" />
          
          <Box mt={4}>
            <Typography variant="h5" component="h2" gutterBottom className="text-gray-800 font-semibold mb-4">
              Order Summary
            </Typography>
            <List>
              <ListItem disablePadding className="mb-2">
                <ListItemText 
                  primary="Order ID" 
                  secondary="#ORD-2023-06-15-001" 
                  primaryTypographyProps={{ className: "text-gray-600" }}
                  secondaryTypographyProps={{ className: "font-medium text-gray-900" }}
                />
              </ListItem>
              <ListItem disablePadding className="mb-2">
                <ListItemText 
                  primary="Date" 
                  secondary={new Date().toLocaleDateString('en-US', { year: 'numeric', month: 'long', day: 'numeric' })} 
                  primaryTypographyProps={{ className: "text-gray-600" }}
                  secondaryTypographyProps={{ className: "font-medium text-gray-900" }}
                />
              </ListItem>
              <ListItem disablePadding className="mb-2">
                <ListItemText 
                  primary="Total Amount" 
                  secondary="$99.99" 
                  primaryTypographyProps={{ className: "text-gray-600" }}
                  secondaryTypographyProps={{ className: "font-medium text-gray-900" }}
                />
              </ListItem>
            </List>
          </Box>
          
          <Box mt={6}>
            <Button 
              variant="contained" 
              color="primary" 
              href="/"
              size="large"
              className="bg-gray-800 hover:bg-gray-700 text-white px-8 py-3 rounded-full transition-all duration-300 ease-in-out transform hover:scale-105"
            >
              Return to Homepage
            </Button>
          </Box>
        </StyledPaper>
      </Container>
    </ProfessionalBackground>
  );
};

export default SuccessPage;