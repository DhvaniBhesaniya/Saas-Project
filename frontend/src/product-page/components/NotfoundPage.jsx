import Box from "@mui/material/Box";
import Button from "@mui/material/Button";
import Container from "@mui/material/Container";
import Typography from "@mui/material/Typography";
import { Link } from "react-router-dom";

export default function NotFound() {

  return (
    <Box
      sx={{
        display: "flex",
        minHeight: "100vh",
        alignItems: "center",
        justifyContent: "center",
        textAlign: "center",
        bgcolor: "background.default",
        p: 3,
      }}
    >
      <Container maxWidth="sm">
        <Typography variant="h3" sx={{ mb: 2 }}>
          Sorry, page not found!
        </Typography>
        <Typography sx={{ color: "text.secondary", mb: 3 }}>
          Sorry, we couldn’t find the page you’re looking for. Perhaps you’ve
          mistyped the URL? Be sure to check your spelling.
        </Typography>
        <Box
          component="img"
          src="/src/assets/illustration-404.svg"
          alt="404 Not Found"
          sx={{
            width: 320,
            height: "auto",
            mb: { xs: 5, sm: 10 },
            mx: "auto", // Center the image horizontally
          }}
        />
        <Button
          component={Link}
          to="/product?type=s"
          size="large"
          variant="contained"
          color="primary"
        >
          Go to home
        </Button>
      </Container>
    </Box>
  );
}
