import React, { useState, useEffect, useRef } from "react";
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
import { RiEditLine } from "react-icons/ri";

import { styled } from "@mui/system";
import { Alert } from "@mui/material";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Link } from "react-router-dom";
import { formatMemberSinceDate } from "../../utils/date";
import useUpdateUserProfile from "../../api-service/ApiRequest"

// Styled components for custom styling
const ProfilePaper = styled(Paper)(({ theme }) => ({
  padding: theme.spacing(3),
  marginBottom: theme.spacing(3),
}));

const SectionTitle = styled(Typography)(({ theme }) => ({
  marginBottom: theme.spacing(2),
}));

export default function UserProfilePage() {
  const [userData, setUserData] = useState(null);

  const profileImgRef = useRef(null);
  const [profileImg, setProfileImg] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const [successMessage, setSuccessMessage] = useState("");
  const { data: authUser } = useQuery({ queryKey: ["authUser"] });

  const { isUpdatingProfile, updateProfile, updateError } =
    useUpdateUserProfile();

  const handleImgChange = (e) => {
    const file = e.target.files[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = () => {
        setProfileImg(reader.result);
      };
      reader.readAsDataURL(file);
    }
  };

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
  }, [userData, authUser]);

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

  const [formData, setFormData] = useState({
    name: "",
    email: "",
    username: "",
    currentPassword: "",
    newPassword: "",
  });

  // console.log(formData);

  const handlePasswordChange = async (e) => {
    e.preventDefault();
    if (newPassword !== confirmPassword) {
      setError("New passwords do not match");
      return;
    }
    setLoading(true);
    try {
      // Implement password change logic here
      await updateProfile(formData);
      if (!updateError) {
        setConfirmPassword("");
        setNewPassword("");
        setCurrentPassword("");
        setFormData({
          name: "",
          email: "",
          username: "",
          currentPassword: "",
          newPassword: "",
        });
        setSuccessMessage("Password changed successfully");
      }
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
              <input
                type="file"
                hidden
                accept="image/*"
                ref={profileImgRef}
                onChange={(e) => handleImgChange(e)}
              />
              {/* <Grid item>
                <Avatar
                  alt={userData?.name}
                  src={
                    profileImg ||
                    userData?.profileImg ||
                    "/avatar-placeholder.png"
                  }
                  sx={{ width: 100, height: 100 }}
                  onClick={() => profileImgRef.current.click()}
                /> 
                
              </Grid>*/}

              <Grid item>
                <div style={{ position: "relative", display: "inline-block" }}>
                  {/* Avatar Component */}
                  <Avatar
                    alt={userData?.name}
                    src={
                      profileImg ||
                      userData?.profileImg ||
                      "/avatar-placeholder.png"
                    }
                    sx={{ width: 100, height: 100 }}
                  />
                  {/* Edit Icon Overlay */}
                  <RiEditLine
                    onClick={() => profileImgRef.current.click()}
                    style={{
                      position: "absolute",
                      bottom: 0,
                      right: 0,
                      fontSize: "1.5rem", // Adjust the size of the icon
                      backgroundColor: "grey",
                      borderRadius: "50%",
                      padding: "4px",
                      boxShadow: "0 0 5px rgba(0,0,0,0.3)", // Optional shadow
                      cursor: "pointer",
                    }}
                  />
                </div>
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
              <Grid item sx={{ ml: 9 }}>
                <Button
                  variant="contained"
                  component="label"
                  onClick={async (e) => {
                    e.preventDefault();
                    await updateProfile({ profileImg });
                    setProfileImg(null);
                  }}
                >
                  {isUpdatingProfile ? "Changing..." : "Update"}
                </Button>
              </Grid>
            </Grid>
          </ProfilePaper>

          {/* Editable User Details */}
          <ProfilePaper elevation={3}>
            <SectionTitle variant="h6">Personal Information</SectionTitle>
            <form>
              <Grid container spacing={3}>
                <Grid item xs={12} sm={6}>
                  <TextField
                    fullWidth
                    name="name"
                    label="Name"
                    value={name}
                    onChange={(e) => {
                      setName(e.target.value);
                      setFormData({
                        ...formData,
                        [e.target.name]: e.target.value,
                      });
                    }}
                  />
                </Grid>
                <Grid item xs={12} sm={6}>
                  <TextField
                    fullWidth
                    label="Email"
                    type="email"
                    name="email"
                    value={email}
                    onChange={(e) => {
                      setEmail(e.target.value);
                      setFormData({
                        ...formData,
                        [e.target.name]: e.target.value,
                      });
                    }}
                  />
                </Grid>
                <Grid item xs={12} sm={6}>
                  <TextField
                    fullWidth
                    label="Username"
                    value={username}
                    name="username"
                    onChange={(e) => {
                      setUsername(e.target.value);
                      setFormData({
                        ...formData,
                        [e.target.name]: e.target.value,
                      });
                    }}
                  />
                </Grid>
                <Grid item xs={12}>
                  <Button
                    type="submit"
                    variant="contained"
                    color="primary"
                    onClick={async (e) => {
                      e.preventDefault();
                      await updateProfile(formData);
                      setFormData({
                        name: "",
                        email: "",
                        username: "",
                        currentPassword: "",
                        newPassword: "",
                      });
                    }}
                  >
                    {isUpdatingProfile ? "Updating..." : "Update Profile"}
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
                    name="currentPassword"
                    value={currentPassword}
                    onChange={(e) => {
                      setCurrentPassword(e.target.value);
                      setFormData({
                        ...formData,
                        [e.target.name]: e.target.value,
                      });
                    }}
                  />
                </Grid>
                <Grid item size={6}>
                  <TextField
                    fullWidth
                    label="New Password"
                    type="password"
                    name="newPassword"
                    value={newPassword}
                    onChange={(e) => {
                      setNewPassword(e.target.value);
                      setFormData({
                        ...formData,
                        [e.target.name]: e.target.value,
                      });
                    }}
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
                  <Button
                    type="submit"
                    variant="contained"
                    color="primary"
                    // onClick={async (e) => {
                    //   e.preventDefault();
                    //   await updateProfile(formData);

                    // }}
                  >
                    {isUpdatingProfile ? "Updating..." : "Change Passowrd"}
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
