import * as React from "react";
import { styled, alpha } from "@mui/material/styles";
import Box from "@mui/material/Box";
import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Button from "@mui/material/Button";
import IconButton from "@mui/material/IconButton";
import Container from "@mui/material/Container";
import Divider from "@mui/material/Divider";
import MenuItem from "@mui/material/MenuItem";
import Drawer from "@mui/material/Drawer";
import MenuIcon from "@mui/icons-material/Menu";
import CloseRoundedIcon from "@mui/icons-material/CloseRounded";
import Sitemark from "./SitemarkIcon";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Link, useNavigate } from "react-router-dom";
import { Menu } from "@mui/material";
import toast from "react-hot-toast";
const StyledToolbar = styled(Toolbar)(({ theme }) => ({
  display: "flex",
  alignItems: "center",
  justifyContent: "space-between",
  flexShrink: 0,
  borderRadius: `calc(${theme.shape.borderRadius}px + 8px)`,
  backdropFilter: "blur(24px)",
  border: "1px solid",
  borderColor: theme.palette.divider,
  backgroundColor: alpha(theme.palette.background.default, 0.4),
  boxShadow: theme.shadows[1],
  padding: "8px 12px",
}));

export default function AppAppBar() {
  const navigate = useNavigate();
  const [anchorEl, setAnchorEl] = React.useState(null);
  const openn = Boolean(anchorEl);

  // Function to handle menu opening
  const handleClick = (event) => {
    setAnchorEl(event.currentTarget);
  };

  // Function to close the menu
  const handleClose = () => {
    setAnchorEl(null);
  };

  const queryClient = useQueryClient();
  const { mutate: logout } = useMutation({
    mutationFn: async () => {
      try {
        const response = await fetch("/api/user/logout", {
          method: "POST",
        });
        const data = await response.json();
        console.log(data);
      } catch (error) {
        console.log(error);
        throw new Error(error);
      }
    },
    onSuccess: () => {
      // Manually set authUser to null (immediate UI change)
      queryClient.setQueryData(["authUser"], null);

      // Show success message
      toast.success("Logged out successfully");
    },
    onError: () => {
      toast.error("Failed to logout");
    },
  });

  // Function to handle scrolling to a section
  const scrollToSection = (sectionId) => {
    const section = document.getElementById(sectionId);
    if (section) {
      section.scrollIntoView({ behavior: "smooth" });
    }
  };
  const [open, setOpen] = React.useState(false);
  const { data: authUser } = useQuery({ queryKey: ["authUser"] });
  const [userData, setUserData] = React.useState(null);

  React.useEffect(() => {
    console.log(authUser);
    setUserData(authUser);
  }, [authUser,userData]);

  const toggleDrawer = (newOpen) => () => {
    setOpen(newOpen);
  };

  return (
    <AppBar
      position="fixed"
      sx={{
        boxShadow: 0,
        bgcolor: "transparent",
        backgroundImage: "none",
        mt: 10,
      }}
    >
      <Container maxWidth="lg">
        <StyledToolbar variant="dense" disableGutters>
          <Box
            sx={{ flexGrow: 1, display: "flex", alignItems: "center", px: 0 }}
          >
            <Link to="/">
              <Sitemark />
            </Link>

            <Box sx={{ display: { xs: "none", md: "flex" } }}>
              <Button
                variant="text"
                color="info"
                size="small"
                onClick={() => scrollToSection("features")}
              >
                Features
              </Button>
              <Button
                variant="text"
                color="info"
                size="small"
                onClick={() => navigate("/product")}
              >
                Products
              </Button>
              <Button
                variant="text"
                color="info"
                size="small"
                onClick={() => scrollToSection("testimonials")}
              >
                Testimonials
              </Button>
              <Button
                variant="text"
                color="info"
                size="small"
                onClick={() => scrollToSection("highlights")}
              >
                Highlights
              </Button>
              <Button
                variant="text"
                color="info"
                size="small"
                onClick={() => scrollToSection("pricing")}
              >
                Pricing
              </Button>
              <Button
                variant="text"
                color="info"
                size="small"
                sx={{ minWidth: 0 }}
                onClick={() => scrollToSection("faq")}
              >
                FAQ
              </Button>
              <Button
                variant="text"
                color="info"
                size="small"
                sx={{ minWidth: 0 }}
                onClick={() => navigate("/blog")}
              >
                Blog
              </Button>
            </Box>
          </Box>
          <Box
            sx={{
              display: { xs: "none", md: "flex" },
              gap: 1,
              alignItems: "center",
            }}
          >
            {authUser ? (
              <>
                <Button
                  color="primary"
                  variant="text"
                  size="small"
                  onClick={handleClick} // Opens the dropdown
                >
                  Hello, {authUser?.name}
                </Button>
                <Menu anchorEl={anchorEl} open={openn} onClose={handleClose}>
                  <MenuItem component={Link} to="/product?type=s">
                    My Profile
                  </MenuItem>
                  <MenuItem
                    onClick={() => {
                      logout(); // Trigger logout
                    }}
                  >
                    Logout
                  </MenuItem>
                </Menu>
              </>
            ) : (
              <>
                <Button
                  color="primary"
                  variant="text"
                  size="small"
                  onClick={() => navigate("/login?mode=login")}
                >
                  Sign in
                </Button>
                <Button
                  color="primary"
                  variant="contained"
                  size="small"
                  onClick={() => navigate("/login?mode=signup")}
                >
                  Sign up
                </Button>
              </>
            )}
          </Box>
          <Box sx={{ display: { sm: "flex", md: "none" } }}>
            <IconButton aria-label="Menu button" onClick={toggleDrawer(true)}>
              <MenuIcon />
            </IconButton>
            <Drawer anchor="top" open={open} onClose={toggleDrawer(false)}>
              <Box sx={{ p: 2, backgroundColor: "background.default" }}>
                <Box
                  sx={{
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "space-between",
                  }}
                >
                  <IconButton onClick={toggleDrawer(false)}>
                    <CloseRoundedIcon />
                  </IconButton>
                </Box>
                <Divider sx={{ my: 3 }} />
                <MenuItem>Features</MenuItem>
                <MenuItem>Testimonials</MenuItem>
                <MenuItem>Highlights</MenuItem>
                <MenuItem>Pricing</MenuItem>
                <MenuItem>FAQ</MenuItem>
                <MenuItem onClick={() => navigate("/blog")}>Blog</MenuItem>
                {authUser ? (
                  <MenuItem>Hello, {authUser?.name}</MenuItem>
                ) : (
                  <>
                    <MenuItem>
                      <Button
                        color="primary"
                        variant="contained"
                        fullWidth
                        onClick={() => navigate("/login?mode=login")}
                      >
                        Sign up
                      </Button>
                    </MenuItem>
                    <MenuItem>
                      <Button
                        color="primary"
                        variant="outlined"
                        fullWidth
                        o
                        onClick={() => navigate("/login?mode=signup")}
                      >
                        Sign in
                      </Button>
                    </MenuItem>
                  </>
                )}
              </Box>
            </Drawer>
          </Box>
        </StyledToolbar>
      </Container>
    </AppBar>
  );
}
