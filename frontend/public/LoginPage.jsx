import { useState } from "react";
import { Link } from "react-router-dom";

import XSvg from "../../../components/svgs/X";

// import { MdOutlineMail, MdOutlinePerson } from "react-icons/md";
import {
  MdOutlineVisibility,
  MdOutlineVisibilityOff,
  MdPassword,
  MdVisibility,
  MdVisibilityOff,
} from "react-icons/md";
import { FaUser } from "react-icons/fa";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import TextField from "@mui/material/TextField";
import AccountCircle from "@mui/icons-material/AccountCircle";
import { Box, IconButton, InputAdornment } from "@mui/material";
import toast from "react-hot-toast";

const LoginPage = () => {
  const [formData, setFormData] = useState({
    username: "",
    password: "",
  });
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [email, setEmailForReset] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  // console.log(email)

  const queryClient = useQueryClient();

  const {
    mutate: logigMutation,
    isPending,
    isError,
    error,
  } = useMutation({
    mutationFn: async ({ username, password }) => {
      try {
        const response = await fetch("/api/auth/login", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            username,
            password,
          }),
        });
        const data = await response.json();
        if (!response.ok) throw new Error(data.error || "Failed to login");
        // if (data.error) throw new Error(data.error);
        // console.log(data);
        return data;
      } catch (error) {
        console.log(error);
        // toast.error(error.message);
        throw error;
      }
    },
    onSuccess: (data) => {
      // console.log(data);
      //   toast.success("Logged in successfully");
      // refetch the authUser
      queryClient.refetchQueries({ querykey: ["authUser"] });
    },
  });

  const handleSubmit = (e) => {
    e.preventDefault();
    // console.log(formData);
    logigMutation(formData);
  };

  const handleInputChange = (e) => {
    setFormData({ ...formData, [e.target.name]: e.target.value });
  };

  const [showPassword, setShowPassword] = useState(false);

  const handleClickShowPassword = () => setShowPassword((prev) => !prev);

  const handleOpenDialog = () => setIsDialogOpen(true);
  const handleCloseDialog = () => setIsDialogOpen(false);
  const handleEmailChange = (e) => setEmailForReset(e.target.value);

  const handleSendClick = async () => {
    // Logic to handle sending the reset link
    // toast.success("Password reset link sent");
    // setEmailForReset("");
    // handleCloseDialog();

    setIsLoading(true);
    try {
      const response = await fetch("/api/auth/forgot-password", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ email }),
      });

      if (!response.ok) {
        throw new Error("Failed to send reset link");
      }

      const data = await response.json();
      toast.success("Password reset link sent to your email!");
      setEmailForReset("");
      handleCloseDialog();
    } catch (error) {
      toast.error(error.message || "Something went wrong!");
    } finally {
      setIsLoading(false);
    }
  };

  //   const isError = false;

  return (
    <div className="max-w-screen-xl mx-auto flex h-screen">
      <div className="flex-1 hidden lg:flex items-center  justify-center">
        <XSvg className="lg:w-2/3 fill-white" />
      </div>
      <div className="flex-1 flex flex-col justify-center items-center">
        <form className="flex gap-4 flex-col" onSubmit={handleSubmit}>
          <XSvg className="w-24 lg:hidden fill-white" />
          <h1 className="text-4xl font-extrabold text-white">{"Let's"} go.</h1>
          {/* <label className="input input-bordered rounded flex items-center gap-2">
            <FaUser />
            <input
              type="text"
              className="grow"
              placeholder="Username / Email"
              name="username"
              onChange={handleInputChange}
              value={formData.username}
            />
          </label>  */}
          <Box sx={{ display: "flex", alignItems: "flex-end", mb: 2 }}>
            <AccountCircle sx={{ color: "white", mr: 1, my: 0.5 }} />
            <TextField
              id="input-with-sx"
              label="Username / Email"
              variant="standard"
              name="username"
              value={formData.username}
              onChange={handleInputChange}
              fullWidth
              required
              slotProps={{
                input: {
                  sx: {
                    color: "white",
                    "& .MuiInput-underline:before": {
                      borderBottomColor: "grey", // Grey underline by default
                    },
                    "& .MuiInput-underline:after": {
                      borderBottomColor: "grey", // Yellow underline on focus
                    },
                  },
                },
                inputLabel: {
                  sx: {
                    color: "grey", // Grey label by default
                    "&.Mui-focused": {
                      color: "gold", // Yellow label on focus
                    },
                  },
                },
              }}
            />
          </Box>

          {/* <label className="input input-bordered rounded flex items-center gap-2">
            <MdPassword />
            <input
              type="password"
              className="grow"
              placeholder="Password"
              name="password"
              onChange={handleInputChange}
              value={formData.password}
            />
          </label> */}

          <Box sx={{ display: "flex", alignItems: "flex-end", mb: 2 }}>
            <MdPassword
              style={{
                color: "white",
                marginRight: "15px",
                marginBottom: "4px",
                marginLeft: "5px",
              }}
            />
            <TextField
              id="password-input"
              label="Password"
              variant="standard"
              name="password"
              required
              type={showPassword ? "text" : "password"}
              value={formData.password}
              onChange={handleInputChange}
              fullWidth
              slotProps={{
                input: {
                  sx: { color: "white", borderBottomColor: "grey" }, // Grey underline for the input field which is not working
                  endAdornment: (
                    <InputAdornment position="end">
                      <IconButton onClick={handleClickShowPassword} edge="end">
                        {showPassword ? (
                          <MdOutlineVisibilityOff style={{ color: "white" }} />
                        ) : (
                          <MdOutlineVisibility style={{ color: "white" }} />
                        )}
                      </IconButton>
                    </InputAdornment>
                  ),
                },
                inputLabel: {
                  sx: {
                    color: "grey", // Grey label by default
                    "&.Mui-focused": {
                      color: "gold", // Yellow label on focus
                    },
                  },
                },
              }}
            />
          </Box>
          <button className="btn rounded-full btn-primary text-white">
            {isPending ? "Loading..." : "Login"}
          </button>
          {isError && <p className="text-red-500">Error : {error.message}</p>}
        </form>
        <div className="flex flex-col gap-2 mt-4 items-center">
          {/* Text with "Don't have an account?" and "Forgot password?" */}
          <div className="flex justify-between w-full text-xs text-white gap-2">
            <p>{"Don't"} have an account ?</p>
            <Link
              // to="/forgot-password"
              onClick={(e) => {
                e.preventDefault();
                handleOpenDialog();
              }}
            >
              <p className="text-blue-500 cursor-pointer">Forgot password ?</p>
            </Link>
          </div>

          {/* Sign Up Button */}
          <Link to="/signup" className="w-full">
            <button className="btn rounded-full btn-primary text-white btn-outline w-full">
              Sign up
            </button>
          </Link>
        </div>

        {isDialogOpen && (
          <>
            <div className="fixed inset-0 bg-black bg-opacity-50 backdrop-blur-sm z-40"></div>
            <dialog open className="modal z-50">
              <div className="modal-box">
                <h3 className="font-bold text-lg">Reset Password</h3>
                <p className="py-4">
                  Enter the email which was provided at the time of creating
                  account. We will send you a link to regenerate the password.
                </p>
                <TextField
                  id="resetEmail"
                  label="Email"
                  variant="outlined"
                  type="email"
                  value={email}
                  onChange={handleEmailChange}
                  fullWidth
                  slotProps={{
                    input: {
                      sx: {
                        color: "white",
                      },
                    },
                    inputLabel: {
                      sx: { color: "grey" },
                    },
                    root: {
                      sx: {
                        "& .MuiOutlinedInput-root": {
                          borderRadius: "4px",
                          height: "40px",
                        },
                        "& .MuiOutlinedInput-notchedOutline": {
                          borderColor: "grey",
                        },
                        "& .Mui-focused .MuiOutlinedInput-notchedOutline": {
                          borderColor: "gold",
                        },
                        "& .Mui-focused .MuiInputLabel-root": {
                          color: "gold",
                        },
                      },
                    },
                  }}
                />
                <div className="modal-action">
                  <button
                    className="btn"
                    onClick={handleSendClick}
                    disabled={!email}
                  >
                    {isLoading ? "Sending..." : "Send"}
                  </button>
                  <button className="btn" onClick={handleCloseDialog}>
                    Cancel
                  </button>
                </div>
              </div>
            </dialog>
          </>
        )}
      </div>
    </div>
  );
};
export default LoginPage;
