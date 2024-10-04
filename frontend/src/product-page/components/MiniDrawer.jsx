import * as React from "react";
import { styled, useTheme } from "@mui/material/styles";
import Box from "@mui/material/Box";
import MuiDrawer from "@mui/material/Drawer";
import MuiAppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import List from "@mui/material/List";
import CssBaseline from "@mui/material/CssBaseline";
import Typography from "@mui/material/Typography";
import Divider from "@mui/material/Divider";
import IconButton from "@mui/material/IconButton";
import MenuIcon from "@mui/icons-material/Menu";
import ChevronLeftIcon from "@mui/icons-material/ChevronLeft";
import ChevronRightIcon from "@mui/icons-material/ChevronRight";
import ListItem from "@mui/material/ListItem";
import ListItemButton from "@mui/material/ListItemButton";
import ListItemIcon from "@mui/material/ListItemIcon";
import ListItemText from "@mui/material/ListItemText";
// import InboxIcon from "@mui/icons-material/MoveToInbox";
// import MailIcon from "@mui/icons-material/Mail";
import AccountSlots from "./AccountSlots";
import TranslationPlatform from "./TranslationPlatform";
import { BsTranslate } from "react-icons/bs";
import { CgProfile } from "react-icons/cg";
import { CiSettings } from "react-icons/ci";
import { FiHelpCircle } from "react-icons/fi";
import CardAlert from "./CardAlert";
import AutoAwesomeRoundedIcon from "@mui/icons-material/AutoAwesomeRounded";
import UserProfile from "./UserProfile";
import { useQuery } from "@tanstack/react-query";
import { Avatar } from "@mui/material";
import { useSearchParams } from "react-router-dom";
import NotFound from "./NotfoundPage";

const drawerWidth = 200;

const openedMixin = (theme) => ({
  width: drawerWidth,
  transition: theme.transitions.create("width", {
    easing: theme.transitions.easing.sharp,
    duration: theme.transitions.duration.enteringScreen,
  }),
  overflowX: "hidden",
  marginTop: "51px", // Added space to push the drawer below the AppBar
});

const closedMixin = (theme) => ({
  transition: theme.transitions.create("width", {
    easing: theme.transitions.easing.sharp,
    duration: theme.transitions.duration.leavingScreen,
  }),
  overflowX: "hidden",
  width: `calc(${theme.spacing(7)} + 1px)`,
  [theme.breakpoints.up("sm")]: {
    width: `calc(${theme.spacing(8)} + 1px)`,
  },
  marginTop: "51px", // Added space to push the drawer below the AppBar
});

const DrawerHeader = styled("div")(({ theme }) => ({
  display: "flex",
  alignItems: "center",
  justifyContent: "flex-end",
  padding: theme.spacing(0, 1),
  ...theme.mixins.toolbar,
}));

const AppBar = styled(MuiAppBar, {
  shouldForwardProp: (prop) => prop !== "open",
})(({ theme, open }) => ({
  zIndex: theme.zIndex.drawer + 1,
  transition: theme.transitions.create(["width", "margin"], {
    easing: theme.transitions.easing.sharp,
    duration: theme.transitions.duration.leavingScreen,
  }),
  ...(open && {
    marginLeft: drawerWidth,
    width: `calc(100% - ${drawerWidth}px)`,
    transition: theme.transitions.create(["width", "margin"], {
      easing: theme.transitions.easing.sharp,
      duration: theme.transitions.duration.enteringScreen,
    }),
  }),
}));

const Drawer = styled(MuiDrawer, {
  shouldForwardProp: (prop) => prop !== "open",
})(({ theme, open }) => ({
  width: drawerWidth,
  flexShrink: 0,
  whiteSpace: "nowrap",
  boxSizing: "border-box",
  ...(open && {
    ...openedMixin(theme),
    "& .MuiDrawer-paper": openedMixin(theme),
  }),
  ...(!open && {
    ...closedMixin(theme),
    "& .MuiDrawer-paper": closedMixin(theme),
  }),
}));

export default function MiniDrawer() {
  const theme = useTheme();
  const [open, setOpen] = React.useState(false);
  const { data: authUser } = useQuery({ queryKey: ["authUser"] });
  // Add state to store the selected product
  const [selectedProduct, setSelectedProduct] = React.useState("AI-Translator");
  const [selectedSetting, setSelectedSetting] = React.useState("User");
  const [currentSelect, setCurrentSelect] = React.useState("p"); // p or s

  const [searchParams] = useSearchParams();
  const page_type = searchParams.get("type");

  // Perform actions based on the "mode"
  React.useEffect(() => {
    if (page_type === "s") {
      // Handle signup mode logic
      setCurrentSelect(page_type);
      setSelectedSetting("User");
    } else {
      // Handle login mode logic
      setCurrentSelect("p");
    }
  }, [page_type]);

  const handleDrawerOpen = () => {
    setOpen(true);
  };

  const handleDrawerClose = () => {
    setOpen(false);
  };

  return (
    <Box sx={{ display: "flex" }}>
      <CssBaseline />
      {/* AppBar remains on top */}
      <AppBar position="fixed" open={open} sx={{ mt: 6.5 }}>
        <Toolbar>
          <IconButton
            color="inherit"
            aria-label="open drawer"
            onClick={handleDrawerOpen}
            edge="start"
            sx={{
              marginRight: 5,
              ...(open && { display: "none" }),
            }}
          >
            <MenuIcon />
          </IconButton>
          <Typography variant="h6" noWrap component="div">
            {currentSelect === "p"
              ? `Product / ${selectedProduct}`
              : `Setting / ${selectedSetting}`}
          </Typography>
        </Toolbar>
      </AppBar>

      {/* Drawer will now start under the AppBar */}
      <Drawer variant="permanent" open={open}>
        <DrawerHeader>
          <IconButton onClick={handleDrawerClose}>
            {theme.direction === "rtl" ? (
              <ChevronRightIcon />
            ) : (
              <ChevronLeftIcon />
            )}
          </IconButton>
        </DrawerHeader>
        <Divider />
        {/* Update the ListItemButton to handle click events */}
        <List>
          {["AI-Translator", "AI- Replace", "Product3", "Product4"].map(
            (text, index) => (
              <ListItem key={text} disablePadding sx={{ display: "block" }}>
                <ListItemButton
                  sx={{
                    minHeight: 48,
                    justifyContent: open ? "initial" : "center",
                    px: 2.5,
                  }}
                  onClick={() => {
                    setSelectedProduct(text);
                    setCurrentSelect("p");
                  }} // Set the selected product on click
                >
                  <ListItemIcon
                    sx={{
                      minWidth: 0,
                      mr: open ? 3 : "auto",
                      justifyContent: "center",
                    }}
                  >
                    {/* <img
                      src={`src/assets/products-icons/${text}.png`}

                      alt={text}
                      style={{ width: "1.5rem", height: "1.5rem" }}
                    /> */}
                    <BsTranslate style={{ fontSize: "1.3rem" }} />
                  </ListItemIcon>
                  <ListItemText primary={text} sx={{ opacity: open ? 1 : 0 }} />
                </ListItemButton>
              </ListItem>
            )
          )}
        </List>
        <Divider />
        <List>
          {["User", "Setting", "Help"].map((text, index) => (
            <ListItem key={text} disablePadding sx={{ display: "block" }}>
              <ListItemButton
                sx={{
                  minHeight: 48,
                  justifyContent: open ? "initial" : "center",
                  px: 2.5,
                }}
                onClick={() => {
                  setSelectedSetting(text);
                  setCurrentSelect("s");
                }} // Set the selected product on click
              >
                <ListItemIcon
                  sx={{
                    minWidth: 0,
                    mr: open ? 3 : "auto",
                    justifyContent: "center",
                  }}
                >
                  {index === 0 ? (
                    <CgProfile style={{ fontSize: "1.3rem" }} />
                  ) : index === 1 ? (
                    <CiSettings style={{ fontSize: "1.3rem" }} />
                  ) : (
                    index === 2 && (
                      <FiHelpCircle style={{ fontSize: "1.3rem" }} />
                    )
                  )}
                </ListItemIcon>
                <ListItemText primary={text} sx={{ opacity: open ? 1 : 0 }} />
              </ListItemButton>
            </ListItem>
          ))}
        </List>

        <Divider />

        {/* usage tracker tab */}
        <Box className="mt-auto rounded">
          <ListItem disablePadding sx={{ display: "block" }}>
            <ListItemButton
              sx={{
                minHeight: 48,
                justifyContent: open ? "initial" : "center",
                px: 2,
                py: 0.5,
              }}
            >
              {open ? (
                <CardAlert />
              ) : (
                <AutoAwesomeRoundedIcon fontSize="small" sx={{ mr: 1 }} />
              )}
            </ListItemButton>
          </ListItem>
        </Box>

        {/*  profile tab */}
        <Box className="mt-4 mb-16 mx-2  p-0.1 rounded">
          <ListItem disablePadding sx={{ display: "block" }}>
            <ListItemButton
              sx={{
                minHeight: 48,
                justifyContent: open ? "initial" : "center",
                px: 2.5,
              }}
              onClick={() => {
                setCurrentSelect("s");
                setSelectedSetting("User");
              }}
            >
              <ListItemIcon
                sx={{
                  minWidth: 0,
                  mr: open ? 3 : "auto",
                  justifyContent: "center",
                }}
              >
                {/* <AccountSlots /> */}
                <Avatar
                  alt="Remy Sharp"
                  src={authUser?.profileImg || "/goku.jpg"}
                />
              </ListItemIcon>
              <ListItemText primary="Profile" sx={{ opacity: open ? 1 : 0 }} />
            </ListItemButton>
          </ListItem>
        </Box>
      </Drawer>

      {/* Content will start below the AppBar and Drawer */}
      <Box component="main" sx={{ flexGrow: 1, p: 3 }}>
        <DrawerHeader />
        <Typography sx={{ marginBottom: 2 }}>
          {currentSelect === "p" ? (
            <>
              {selectedProduct === "AI-Translator" && <TranslationPlatform />}
              {selectedProduct === "AI- Replace" &&
                "project  :  add your clear standing picture.then  side option to upload the pictures of clothes. then those  cloths will be added on your standing  image to see how it looks on you. , for everything for head ,  t-shirt  shirt ...., jeans..., shoes..."}
              {selectedProduct === "Product3" && "Content for Product 3"}
              {selectedProduct === "Product4" && "Content for Product 4"}
            </>
          ) : (
            <>
              {selectedSetting === "User" && <UserProfile />}
              {selectedSetting === "Setting" && "Content for setting"}
              {selectedSetting === "Help" && <NotFound />}
            </>
          )}
        </Typography>
      </Box>
    </Box>
  );
}
