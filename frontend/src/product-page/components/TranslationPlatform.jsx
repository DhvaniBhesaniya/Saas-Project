import React, { useEffect, useState } from "react";
import {
  Button,
  TextField,
  Select,
  MenuItem,
  TextareaAutosize,
  Card,
  CardContent,
  CardHeader,
  Typography,
  Tabs,
  Tab,
  Box,
  LinearProgress,
  Input,
  FormControl,
  InputLabel,
} from "@mui/material";
import { Upload, Send, Download, HighlightOff } from "@mui/icons-material";
import { styled } from "@mui/material/styles";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import {
  getGenaiChat,
  getGenaiDoc,
  getGenaiTranslatedText,
} from "../../api-service/ApiRequest";
import toast from "react-hot-toast";
import { useUsage } from "../../contexts/UsageContext";

// Custom components that don't have direct MUI equivalents
const ScrollArea = ({ children, className }) => (
  <div className={`overflow-auto ${className}`}>{children}</div>
);

const StyledCard = styled(Card)(({ theme }) => ({
  borderRadius: "12px",
  boxShadow: "0 4px 6px rgba(0, 0, 0, 0.1)",
  margin: theme.spacing(2, 0),
}));

const StyledTab = styled(Tab)(({ theme }) => ({
  textTransform: "none",
  minWidth: 0,
  fontWeight: theme.typography.fontWeightRegular,
  marginRight: theme.spacing(4),
  "&.Mui-selected": {
    fontWeight: theme.typography.fontWeightMedium,
  },
}));

const StyledSelect = styled(Select)(({ theme }) => ({
  "& .MuiOutlinedInput-notchedOutline": {
    borderColor: "rgba(0, 0, 0, 0.23)",
  },
}));

const StyledTextArea = styled(TextareaAutosize)(({ theme }) => ({
  width: "100%",
  padding: theme.spacing(1),
  borderRadius: "4px",
  border: "1px solid rgba(0, 0, 0, 0.23)",
  fontFamily: theme.typography.fontFamily,
  fontSize: "1rem",
  "&:focus": {
    outlineColor: theme.palette.primary.main,
  },
}));

const TranslateButton = styled(Button)(({ theme }) => ({
  marginTop: theme.spacing(2),
  marginBottom: theme.spacing(2),
  padding: theme.spacing(1, 3),
}));

export default function TranslationPlatform() {
  const { incrementUsage, isLimitReached } = useUsage();

  const [sourceLanguage, setSourceLanguage] = useState("en");
  const [targetLanguage, setTargetLanguage] = useState("es");
  const [translatedText, setTranslatedText] = useState("");
  const [uploadProgress, setUploadProgress] = useState(0);
  const [chatMessages, setChatMessages] = useState([]);
  const [tabValue, setTabValue] = useState(0);
  const [file, setFile] = useState(null);
  const [downloadUrl, setDownloadUrl] = useState(null); // Store file URL
  const [downloadFileName, setDownloadFileName] = useState(""); // Store file name

  const languages = [
    { value: "en", label: "English" },
    { value: "es", label: "Spanish" },
    { value: "hi", label: "Hindi" },
    { value: "gu", label: "Gujarati" },
    { value: "fr", label: "French" },
    { value: "de", label: "German" },
    { value: "it", label: "Italian" },
    { value: "pt", label: "Portuguese" },
    { value: "ru", label: "Russian" },
    { value: "zh", label: "Chinese" },
    { value: "ja", label: "Japanese" },
    { value: "ko", label: "Korean" },
  ];

  const [textformData, setTextFormData] = React.useState({
    text: "",
    language: "Spanish",
  });

  const queryClient = useQueryClient();

  const { mutate: textMutation, isPending: textLoading } = useMutation({
    mutationFn: async (textformData) => {
      return getGenaiTranslatedText(textformData); // Use the loginUser API function
    },
    onSuccess: (data) => {
      // Handle successful login
      toast.success(data.message);
      setTranslatedText(data.data);
      incrementUsage(); // Increment usage count on success
      // queryClient.fetchQuery({ queryKey: ["authUser"] });
      // navigate("/");
    },
    onError: (error) => {
      console.error("Error in Translating :", error);
      // Handle login error (show toast notification or error message)
    },
  });
  const handleTranslate = (e) => {
    setTranslatedText("");
    e.preventDefault();
    textMutation(textformData);
  };
  const handleClear = (e) => {
    e.preventDefault();
    setTranslatedText("");
    setTextFormData({
      ...textformData,
      text: "",
    });
  };

  // const [chatformData, setChatFormData] = React.useState({
  //   textmessage: "",
  //   language: "Spanish",
  // });

  // useMutation for sending chat message and getting AI response
  const {
    mutate: ChatMutation,
    isPending: ChatLoading,
    isError,
    error,
  } = useMutation({
    mutationFn: async (chatformData) => {
      return getGenaiChat(chatformData); // Use your API function for sending the chat
    },
    onSuccess: (data) => {
      // On success, append AI response to chat
      const aiMessage = data.data || "AI response not available"; // Fallback if no data
      setChatMessages((prevMessages) => [
        ...prevMessages,
        { text: aiMessage, sender: "ai" },
      ]);
      incrementUsage(); // Increment usage count on success
      toast.success(data.message);
    },
    onError: (error) => {
      // console.error("Error in AI chat:", error);
      toast.error("Failed to get AI response.");
    },
  });

  // Handle form submission
  const handleChatSubmit = (event) => {
    event.preventDefault();
    const message = event.target.message.value;
    if (message.trim()) {
      // Add user message to the chat
      setChatMessages((prevMessages) => [
        ...prevMessages,
        { text: message, sender: "user" },
      ]);

      // Prepare chat data for the mutation
      const selectedLanguage = languages.find(
        (lang) => lang.value === targetLanguage
      );
      const updatedFormData = {
        textmessage: message,
        language: selectedLanguage?.label || targetLanguage,
      };

      // Send the message to the AI and handle response
      ChatMutation(updatedFormData);

      // Reset the form field
      event.target.reset();
    }
  };

  const handleFileUpload = (event) => {
    const selectedFile = event.target.files[0];
    if (selectedFile) {
      setFile(selectedFile);
    }
  };

  const handleUpload = async () => {
    if (!file) {
      toast("Please select a file.", {
        icon: "📁",
        // position: "top-right",
      });
      return;
    }

    if (file.size > 10 * 1024 * 1024) {
      toast.error("File size exceeds 10MB.");
      return;
    }

    let progress = 0;
    const interval = setInterval(() => {
      progress += 10;
      setUploadProgress(progress);
      if (progress >= 100) {
        clearInterval(interval);
      }
    }, 200);

    const selectedLanguage = languages.find(
      (lang) => lang.value === targetLanguage
    );

    // Create FormData to send file and target language
    const docformData = new FormData();
    docformData.append("file", file);
    docformData.append("language", selectedLanguage?.label || targetLanguage);

    DocMutation(docformData);
  };

  const { mutate: DocMutation, isLoading: DocLoading } = useMutation({
    mutationFn: async (docFormData) => {
      return getGenaiDoc(docFormData);
    },
    onSuccess: (data) => {
      // Handle success, create download URL
      // if (uploadProgress === 100) {
      const { blob, fileName } = data;
      const url = window.URL.createObjectURL(blob); // Create URL for download
      setDownloadUrl(url); // Set the download URL in state
      setDownloadFileName(fileName); // Set the file name in state
      setUploadProgress(0); // Reset progress bar after completion
      incrementUsage(); // Increment usage count on success
      toast.success("Document translated successfully!");
      // }
    },
    onError: (error) => {
      // Handle error, show a toast message
      toast.error(`Error: ${error.message}`);
    },
  });

  const handleDownload = () => {
    if (downloadUrl && downloadFileName) {
      const a = document.createElement("a");
      a.href = downloadUrl;
      a.download = downloadFileName;
      a.click();
    }
  };
  const handleClearFile = () => {
    setTargetLanguage("es"); // Reset target language to 'es'
    setFile(null); // Clear the selected file
    setUploadProgress(0); // Reset progress bar after completion

    // Clear the file input field by using its id
    const fileInput = document.getElementById("fileUpload");
    if (fileInput) {
      fileInput.value = null; // Reset the file input
    }

    setDownloadUrl(null); // Optionally reset download URL if necessary
    setDownloadFileName(""); // Optionally reset download file name if necessary
  };

  const handleChatClear = () => {
    setChatMessages([]);
  };

  const { data: authUser } = useQuery({ queryKey: ["authUser"] });

  useEffect(() => {
    // queryClient.fetchQuery({ queryKey: ["authUser"] });
    console.log(authUser);
  }, [authUser]);

  return (
    <div className="container mx-auto p-4">
      <Typography variant="h4" gutterBottom>
        AI-Powered Language Translation Platform
      </Typography>
      <Tabs
        value={tabValue}
        onChange={(e, newValue) => {
          setTabValue(newValue);
          setSourceLanguage("en");
          setTargetLanguage("es");
          setTextFormData({ text: "", language: "Spanish" });
        }}
        aria-label="translation tabs"
        variant="scrollable"
        scrollButtons="auto"
        sx={{ borderBottom: 1, borderColor: "divider", mb: 3 }}
      >
        <StyledTab label="Text Translation" />
        <StyledTab label="Document Translation" />
        <StyledTab label="Chat with AI" />
      </Tabs>
      <Box sx={{ p: 3 }}>
        {tabValue === 0 && (
          <StyledCard>
            <CardHeader
              title="Text Translation"
              subheader="Translate text to selected language."
            />
            <CardContent>
              <Box
                sx={{
                  display: "grid",
                  gridTemplateColumns: "1fr 1fr",
                  gap: 2,
                  mb: 2,
                }}
              >
                {/* <StyledSelect
                  value={sourceLanguage}
                  onChange={(e) => setSourceLanguage(e.target.value)}
                  fullWidth
                  label="Source Language"
                >
                  {languages.map((lang) => (
                    <MenuItem key={lang.value} value={lang.value}>
                      {lang.label}
                    </MenuItem>
                  ))}
                </StyledSelect> */}
                <FormControl fullWidth>
                  <InputLabel>Target Language</InputLabel>
                  <StyledSelect
                    value={targetLanguage}
                    onChange={(e) => {
                      setTargetLanguage(e.target.value);
                      setTextFormData({
                        ...textformData,
                        language: languages.find(
                          (lang) => lang.value === e.target.value
                        ).label,
                      });
                    }}
                    fullWidth
                    label="Target Language"
                  >
                    {languages.map((lang) => (
                      <MenuItem key={lang.value} value={lang.value}>
                        {lang.label}
                      </MenuItem>
                    ))}
                  </StyledSelect>
                </FormControl>
              </Box>
              <StyledTextArea
                minRows={4}
                placeholder="Enter text to translate"
                name="text"
                value={textformData.text}
                onChange={(e) => {
                  setTextFormData({
                    ...textformData,
                    [e.target.name]: e.target.value,
                  });
                }}
              />
              <TranslateButton
                variant="contained"
                onClick={handleTranslate}
                sx={{ marginRight: 2 }}
                disabled={isLimitReached} // Disable if limit reached
              >
                {textLoading ? "Translating ..." : "Translate"}
              </TranslateButton>
              <TranslateButton variant="contained" onClick={handleClear}>
                Clear
              </TranslateButton>
              <Typography variant="subtitle1">Translated Text</Typography>
              <StyledTextArea
                minRows={4}
                value={`${translatedText}`}
                readOnly
              />
            </CardContent>
          </StyledCard>
        )}
        {tabValue === 1 && (
          <Card>
            <CardHeader
              title="Document Translation"
              subheader="Upload and translate entire documents (.doc and .txt only)"
            />
            <CardContent>
              <Box
                sx={{
                  display: "grid",
                  gridTemplateColumns: "1fr 1fr",
                  gap: 2,
                  mb: 2,
                }}
              >
                <FormControl fullWidth>
                  <InputLabel>Target Language</InputLabel>
                  <StyledSelect
                    value={targetLanguage}
                    onChange={(e) => setTargetLanguage(e.target.value)}
                    fullWidth
                    label="Target Language"
                  >
                    {languages.map((lang) => (
                      <MenuItem key={lang.value} value={lang.value}>
                        {lang.label}
                      </MenuItem>
                    ))}
                  </StyledSelect>
                </FormControl>
              </Box>

              <Box sx={{ display: "flex", alignItems: "center", gap: 2 }}>
                <Input
                  type="file"
                  id="fileUpload"
                  onChange={handleFileUpload}
                />
                <Button
                  variant="contained"
                  startIcon={<Upload />}
                  onClick={handleUpload}
                  disabled={
                    (uploadProgress > 0 && uploadProgress < 100) ||
                    isLimitReached
                  } // Disable if limit reached}
                >
                  Upload
                </Button>
                {downloadUrl && (
                  <Button
                    variant="contained"
                    startIcon={<Download />}
                    onClick={handleDownload} // Trigger download when clicked
                  >
                    Download
                  </Button>
                )}

                <Button
                  variant="contained"
                  startIcon={<HighlightOff />}
                  onClick={handleClearFile} // Call handleClear on click
                >
                  Clear
                </Button>
              </Box>

              {uploadProgress > 0 && (
                <LinearProgress
                  variant="determinate"
                  value={uploadProgress}
                  sx={{ mt: 2 }}
                />
              )}
            </CardContent>
          </Card>
        )}

        {tabValue === 2 && (
          <Card>
            <CardHeader
              title="Chat with AI"
              subheader="Chat with AI in any language."
            />
            <CardContent>
              <Box
                sx={{
                  display: "grid",
                  gridTemplateColumns: "1fr 1fr",
                  gap: 2,
                  mb: 2,
                }}
              >
                <FormControl fullWidth>
                  <InputLabel>Ai Language</InputLabel>
                  <StyledSelect
                    value={targetLanguage}
                    onChange={(e) => setTargetLanguage(e.target.value)}
                    fullWidth
                    label="Ai Language"
                  >
                    {languages.map((lang) => (
                      <MenuItem key={lang.value} value={lang.value}>
                        {lang.label}
                      </MenuItem>
                    ))}
                  </StyledSelect>
                </FormControl>
                <TranslateButton
                  variant="contained"
                  onClick={handleChatClear}
                  sx={{ width: 100 }}
                >
                  Clear
                </TranslateButton>
              </Box>

              {/* Chat Display Area */}
              <ScrollArea className="h-[300px] w-full border rounded-md p-4 mb-4">
                {chatMessages.map((msg, index) => (
                  <div
                    key={index}
                    className={`chat ${msg.sender === "user" ? "chat-end" : "chat-start"}`}
                  >
                    <div className="chat-image avatar">
                      <div className="w-10 rounded-full">
                        <img
                          alt={
                            msg.sender === "user" ? "User avatar" : "AI avatar"
                          }
                          src={
                            msg.sender === "user"
                              ? authUser?.profileImg ||
                                "/avatar-placeholder.png" // User image
                              : "/chatbot.png" // AI image
                          }
                        />
                      </div>
                    </div>
                    <div className="chat-bubble break-words max-w-xs">
                      {msg.sender === "ai" && ChatLoading ? (
                        <span className="loading loading-dots loading-md"></span>
                      ) : (
                        msg.text
                      )}
                    </div>
                  </div>
                ))}
              </ScrollArea>

              {/* Chat Input */}
              <form onSubmit={handleChatSubmit} className="flex gap-2">
                <TextField
                  name="message"
                  placeholder="Type your message..."
                  fullWidth
                  multiline
                  rows={1.5}
                  variant="outlined"
                  slotProps={{
                    style: {
                      whiteSpace: "pre-wrap", // Wrap text within TextField
                      overflow: "auto", // Make it scrollable
                      wordBreak: "break-word", // Wrap long words
                    },
                  }}
                />
                <Button
                  type="submit"
                  variant="contained"
                  endIcon={<Send />}
                  disabled={isLimitReached} // Disable if limit reached
                >
                  Send
                </Button>
              </form>
            </CardContent>
          </Card>
        )}
      </Box>
    </div>
  );
}

// const handleUpload = async () => {
//   if (!file) {
//     alert("Please select a file.");
//     return;
//   }

//   if (file.size > 10 * 1024 * 1024) {
//     alert("File size exceeds 10MB.");
//     return;
//   }

//   let progress = 0;
//   const interval = setInterval(() => {
//     progress += 10;
//     setUploadProgress(progress);
//     if (progress >= 100) {
//       clearInterval(interval);
//     }
//   }, 200);

//   // Prepare chat data for the mutation
//   const selectedLanguage = languages.find(
//     (lang) => lang.value === targetLanguage
//   );

//   // Create FormData to send file and target language
//   const formData = new FormData();
//   formData.append("file", file);
//   formData.append("language", selectedLanguage?.label || targetLanguage);

//   try {
//     const response = await fetch("/api/genai/doc", {
//       method: "POST",
//       body: formData,
//     });

//     if (response.ok) {
//       const blob = await response.blob(); // Get the translated file as a blob
//       const url = window.URL.createObjectURL(blob); // Create URL for download
//       const a = document.createElement("a");
//       a.href = url;
//       a.download = file.name; // Same file name, but content is translated
//       a.click();
//       toast.success("Document translated successfully!");
//     } else {
//       const errorData = await response.json();
//       alert(`Error: ${errorData.message}`);
//     }
//   } catch (error) {
//     console.error("Upload error:", error);
//     alert("Failed to upload the document.");
//   }

//   setUploadProgress(0); // Reset progress bar after completion
// };

// {tabValue === 1 && (
//   <Card>
//     <CardHeader
//       title="Document Translation"
//       subheader="Upload and translate entire documents (.doc and .txt only)"
//     />
//     <CardContent>
//       <Box
//         sx={{
//           display: "grid",
//           gridTemplateColumns: "1fr 1fr",
//           gap: 2,
//           mb: 2,
//         }}
//       >
{
  /* <Select fullWidth label="Source Language">
          {languages.map((lang) => (
            <MenuItem key={lang.value} value={lang.value}>
              {lang.label}
            </MenuItem>
          ))}
        </Select> */
}
//         <FormControl fullWidth>
//           <InputLabel>Target Language</InputLabel>
//           <StyledSelect
//             value={targetLanguage}
//             onChange={(e) => setTargetLanguage(e.target.value)}
//             fullWidth
//             label="Target Language"
//           >
//             {languages.map((lang) => (
//               <MenuItem key={lang.value} value={lang.value}>
//                 {lang.label}
//               </MenuItem>
//             ))}
//           </StyledSelect>
//         </FormControl>
//       </Box>
//       <Box sx={{ display: "flex", alignItems: "center", gap: 2 }}>
//         <Input type="file" onChange={handleFileUpload} />
//         <Button variant="contained" startIcon={<Upload />}>
//           Upload
//         </Button>
//       </Box>
//       {uploadProgress > 0 && (
//         <LinearProgress
//           variant="determinate"
//           value={uploadProgress}
//           sx={{ mt: 2 }}
//         />
//       )}
//     </CardContent>
//   </Card>
// )}

// <Card>
// <CardHeader
//   title="Chat with AI"
//   subheader="Chat with AI in any language."
// />
// <CardContent>
//   <Box
//     sx={{
//       display: "grid",
//       gridTemplateColumns: "1fr 1fr",
//       gap: 2,
//       mb: 2,
//     }}
//   >
//     <FormControl fullWidth>
//       <InputLabel>Ai Language</InputLabel>
//       <StyledSelect
//         value={targetLanguage}
//         onChange={(e) => setTargetLanguage(e.target.value)}
//         fullWidth
//         label="Target Language"
//       >
//         {languages.map((lang) => (
//           <MenuItem key={lang.value} value={lang.value}>
//             {lang.label}
//           </MenuItem>
//         ))}
//       </StyledSelect>
//     </FormControl>
//   </Box>

//   <ScrollArea className="h-[300px] w-full border rounded-md p-4 mb-4">
//     {chatMessages.map((msg, index) => (
//       <div
//         key={index}
//         className={`chat ${msg.sender === "user" ? "chat-end" : "chat-start"}`}
//       >
//         <div className="chat-image avatar">
//           <div className="w-10 rounded-full">
//             <img
//               alt={
//                 msg.sender === "user" ? "User avatar" : "AI avatar"
//               }
//               src={
//                 msg.sender === "user"
//                   ? "https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp" // Replace with actual user avatar URL
//                   : "https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp"
//               } // Replace with actual AI avatar URL
//             />
//           </div>
//         </div>
//         <div className="chat-bubble break-words max-w-xs">
//           {msg.text}
//         </div>{" "}

//       </div>
//     ))}
//   </ScrollArea>

//   <form onSubmit={handleChatSubmit} className="flex gap-2">
//     <TextField
//       name="message"
//       placeholder="Type your message..."
//       fullWidth
//       multiline
//       rows={1.5} // This sets the default height of the text area
//       variant="outlined"
//       slotProps={{
//         style: {
//           whiteSpace: "pre-wrap", // Allows text wrapping within the TextField
//           overflow: "auto", // Makes the TextField scrollable when content overflows
//           wordBreak: "break-word", // Forces long words to wrap onto new lines
//         },
//       }}
//     />
//     <Button type="submit" variant="contained" endIcon={<Send />}>
//       Send
//     </Button>
//   </form>
// </CardContent>
// </Card>
