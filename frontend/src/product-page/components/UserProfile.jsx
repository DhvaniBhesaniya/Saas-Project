import React, { useState, useEffect } from "react";
import {
  Avatar,
  Button,
  Container,
  Paper,
  TextField,
  Typography,
  Select,
  MenuItem,
  FormControlLabel,
  Switch,
  List,
  ListItem,
  ListItemText,
  Divider,
  Snackbar,
  CircularProgress,
  Box,
  Card,
  CardContent,
} from "@mui/material";
import Grid from "@mui/material/Grid2";

import { styled } from "@mui/system";
import { Alert } from "@mui/material";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { Link } from "react-router-dom";
import { formatMemberSinceDate } from "../../utils/date";

// Styled components for custom styling
const ProfilePaper = styled(Paper)(({ theme }) => ({
  padding: theme.spacing(3),
  marginBottom: theme.spacing(3),
}));

const SectionTitle = styled(Typography)(({ theme }) => ({
  marginBottom: theme.spacing(2),
}));

const updateUserData = (data) => {
  // Simulating API call
  return new Promise((resolve) => {
    setTimeout(() => {
      console.log("User data updated:", data);
      resolve({ success: true });
    }, 1000);
  });
};

export default function UserProfilePage() {
  const [userData, setUserData] = useState(null);
  // const queryClient = useQueryClient();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const [successMessage, setSuccessMessage] = useState("");
  const { data: authUser } = useQuery({ queryKey: ["authUser"] });

  useEffect(() => {
    // queryClient.fetchQuery({ queryKey: ["authUser"] });
    console.log(authUser);
    setUserData(authUser);
    setName(userData?.name);
    setEmail(userData?.email);
    setUsername(userData?.username);
    setLanguage("en");
    setTimezone("UTC");
    setEmailNotifications(true);
    setPushNotifications(false);
  }, [authUser,userData]);

  // Form state
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [username, setUsername] = useState("");
  const [currentPassword, setCurrentPassword] = useState("");
  const [newPassword, setNewPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [language, setLanguage] = useState("");
  const [timezone, setTimezone] = useState("");
  const [emailNotifications, setEmailNotifications] = useState(false);
  const [pushNotifications, setPushNotifications] = useState(false);

  const handleSubmit = async (e) => {
    e.preventDefault();
    setLoading(true);
    try {
      await updateUserData({
        name,
        email,
        username,
        language,
        timezone,
        notificationPreferences: {
          email: emailNotifications,
          push: pushNotifications,
        },
      });
      setSuccessMessage("Profile updated successfully");
    } catch (err) {
      setError("Failed to update profile");
    } finally {
      setLoading(false);
    }
  };

  const handlePasswordChange = async (e) => {
    e.preventDefault();
    if (newPassword !== confirmPassword) {
      setError("New passwords do not match");
      return;
    }
    setLoading(true);
    try {
      // Implement password change logic here
      setSuccessMessage("Password changed successfully");
    } catch (err) {
      setError("Failed to change password");
    } finally {
      setLoading(false);
    }
  };
  if (loading && !userData) {
    return (
      <Box
        sx={{
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
          height: "100vh",
        }}
      >
        <CircularProgress />
      </Box>
    );
  }

  if (error && !userData) {
    return <Alert severity="error">{error}</Alert>;
  }

  return (
    <Container maxWidth="md">
      {authUser ? (
        <>
          <Typography variant="h4" gutterBottom>
            User Profile
          </Typography>

          {/* User Information Display */}
          <ProfilePaper elevation={3}>
            <Grid container spacing={3} alignItems="center">
              <Grid item>
                <Avatar
                  alt={userData?.name}
                  src={userData?.profileImg}
                  sx={{ width: 100, height: 100 }}
                />
              </Grid>
              <Grid item size={6.8}>
                <Typography variant="h6">{userData?.name}</Typography>
                <Typography variant="body1">{userData?.email}</Typography>
                <Typography variant="body2">
                  {formatMemberSinceDate(userData?.created_at)}
                </Typography>
                <Typography variant="body2">
                  Account Type: {userData?.subscription_plan?.plan_type}
                </Typography>
              </Grid>
              <Grid item>
                <Button variant="contained" component="label">
                  Change Picture
                  <input type="file" hidden />
                </Button>
              </Grid>
            </Grid>
          </ProfilePaper>

          {/* Editable User Details */}
          <ProfilePaper elevation={3}>
            <SectionTitle variant="h6">Personal Information</SectionTitle>
            <form onSubmit={handleSubmit}>
              <Grid container spacing={3}>
                <Grid item xs={12} sm={6}>
                  <TextField
                    fullWidth
                    label="Name"
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                  />
                </Grid>
                <Grid item xs={12} sm={6}>
                  <TextField
                    fullWidth
                    label="Email"
                    type="email"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                  />
                </Grid>
                <Grid item xs={12} sm={6}>
                  <TextField
                    fullWidth
                    label="Username"
                    value={username}
                    onChange={(e) => setUsername(e.target.value)}
                  />
                </Grid>
                <Grid item xs={12}>
                  <Button type="submit" variant="contained" color="primary">
                    Update Profile
                  </Button>
                </Grid>
              </Grid>
            </form>
          </ProfilePaper>

          {/* Password Management */}
          <ProfilePaper elevation={2}>
            <SectionTitle variant="h6">Change Password</SectionTitle>
            <form onSubmit={handlePasswordChange}>
              <Grid container spacing={2}>
                <Grid item size={12}>
                  <TextField
                    fullWidth
                    label="Current Password"
                    type="password"
                    value={currentPassword}
                    onChange={(e) => setCurrentPassword(e.target.value)}
                  />
                </Grid>
                <Grid item size={6}>
                  <TextField
                    fullWidth
                    label="New Password"
                    type="password"
                    value={newPassword}
                    onChange={(e) => setNewPassword(e.target.value)}
                  />
                </Grid>
                <Grid item size={6}>
                  <TextField
                    fullWidth
                    label="Confirm New Password"
                    type="password"
                    value={confirmPassword}
                    onChange={(e) => setConfirmPassword(e.target.value)}
                  />
                </Grid>
                <Grid item xs={12}>
                  <Button type="submit" variant="contained" color="primary">
                    Change Password
                  </Button>
                </Grid>
              </Grid>
            </form>
          </ProfilePaper>

          {/* Account Preferences */}
          <ProfilePaper elevation={3}>
            <SectionTitle variant="h6">Account Preferences</SectionTitle>
            <Grid container spacing={3}>
              <Grid item size={6}>
                <Select
                  fullWidth
                  value={language}
                  onChange={(e) => setLanguage(e.target.value)}
                  displayEmpty
                >
                  <MenuItem value="" disabled>
                    Select Language
                  </MenuItem>
                  <MenuItem value="en">English</MenuItem>
                  <MenuItem value="es">Spanish</MenuItem>
                  <MenuItem value="fr">French</MenuItem>
                </Select>
              </Grid>
              <Grid item size={6}>
                <Select
                  fullWidth
                  value={timezone}
                  onChange={(e) => setTimezone(e.target.value)}
                  displayEmpty
                >
                  <MenuItem value="" disabled>
                    Select Timezone
                  </MenuItem>
                  <MenuItem value="UTC">UTC</MenuItem>
                  <MenuItem value="EST">EST</MenuItem>
                  <MenuItem value="PST">PST</MenuItem>
                </Select>
              </Grid>
              <Grid item xs={12}>
                <FormControlLabel
                  control={
                    <Switch
                      checked={emailNotifications}
                      onChange={(e) => setEmailNotifications(e.target.checked)}
                    />
                  }
                  label="Email Notifications"
                />
              </Grid>
              <Grid item xs={12}>
                <FormControlLabel
                  control={
                    <Switch
                      checked={pushNotifications}
                      onChange={(e) => setPushNotifications(e.target.checked)}
                    />
                  }
                  label="Push Notifications"
                />
              </Grid>
            </Grid>
          </ProfilePaper>

          {/* Subscription Management */}
          <ProfilePaper elevation={3}>
            <SectionTitle variant="h6">Subscription Management</SectionTitle>
            <Typography variant="body1" gutterBottom>
              Current Plan: {userData?.subscription_plan?.plan_type}
            </Typography>
            <Button variant="outlined" color="primary">
              Upgrade Plan
            </Button>
            <Divider sx={{ my: 2 }} />
            <Typography variant="subtitle1" gutterBottom>
              Billing History
            </Typography>
            <List>
              <ListItem>
                <ListItemText
                  primary="Invoice #1234"
                  secondary="Paid on 2023-05-01"
                />
              </ListItem>
              <ListItem>
                <ListItemText
                  primary="Invoice #1235"
                  secondary="Paid on 2023-06-01"
                />
              </ListItem>
            </List>
          </ProfilePaper>

          {/* Connected Accounts */}
          <ProfilePaper elevation={3}>
            <SectionTitle variant="h6">Connected Accounts</SectionTitle>
            <List>
              <ListItem>
                <ListItemText primary="Google" secondary="Connected" />
                <Button variant="outlined" color="secondary">
                  Disconnect
                </Button>
              </ListItem>
              <ListItem>
                <ListItemText primary="Facebook" secondary="Not connected" />
                <Button variant="outlined" color="primary">
                  Connect
                </Button>
              </ListItem>
            </List>
          </ProfilePaper>

          {/* Data and Privacy */}
          <ProfilePaper elevation={3}>
            <SectionTitle variant="h6">Data and Privacy</SectionTitle>
            <Button variant="outlined" color="primary" sx={{ mr: 2 }}>
              Export Data
            </Button>
            <Button variant="outlined" color="primary">
              Privacy Settings
            </Button>
          </ProfilePaper>

          {/* Account Deletion */}
          <ProfilePaper elevation={3}>
            <SectionTitle variant="h6">Account Deletion</SectionTitle>
            <Typography variant="body2" gutterBottom sx={{ pb: 1 }}>
              Deleting your account will permanently remove all your data. This
              action cannot be undone.
            </Typography>
            <Button variant="contained" color="error">
              Delete Account
            </Button>
          </ProfilePaper>

          {/* Two-Factor Authentication */}
          <ProfilePaper elevation={3}>
            <SectionTitle variant="h6">Two-Factor Authentication</SectionTitle>
            <FormControlLabel
              control={<Switch />}
              label="Enable Two-Factor Authentication"
            />
            <Button variant="outlined" color="primary" sx={{ ml: 40 }}>
              Set Up 2FA
            </Button>
          </ProfilePaper>

          {/* Activity Log */}
          <ProfilePaper elevation={3}>
            <SectionTitle variant="h6">Activity Log</SectionTitle>
            <List>
              <ListItem>
                <ListItemText
                  primary="Login from Chrome on Windows"
                  secondary="2023-07-01 10:30 AM"
                />
              </ListItem>
              <ListItem>
                <ListItemText
                  primary="Password changed"
                  secondary="2023-06-28 2:15 PM"
                />
              </ListItem>
            </List>
            <Button variant="outlined" color="primary">
              View All Activity
            </Button>
          </ProfilePaper>

          {/* Success and Error messages */}
          <Snackbar
            open={!!successMessage}
            autoHideDuration={6000}
            onClose={() => setSuccessMessage("")}
          >
            <Alert onClose={() => setSuccessMessage("")} severity="success">
              {successMessage}
            </Alert>
          </Snackbar>
          <Snackbar
            open={!!error}
            autoHideDuration={6000}
            onClose={() => setError(null)}
          >
            <Alert onClose={() => setError(null)} severity="error">
              {error}
            </Alert>
          </Snackbar>
        </>
      ) : (
        <Card style={{ marginTop: "16px" }}>
          <CardContent>
            <Typography variant="h5" component="div">
              <Link to="/login?mode=login">Login to see more settings</Link>
            </Typography>
            <Typography variant="body2" color="text.secondary">
              Please login to access your profile and account settings.
            </Typography>
          </CardContent>
        </Card>
      )}
    </Container>
  );
}
