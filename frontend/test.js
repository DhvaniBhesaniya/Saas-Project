// import  utc  from 'moment-timezone';

// export const convertToIndianTime = (createdAt) => {
//     // First, clean up the createdAt string to a more standard ISO format
//     let [datePart, timePart] = createdAt.split(" ");
//     const [hours, rest] = timePart.split(":");
  
//     // Ensure the hours are zero-padded to two digits
//     const paddedHours = hours.padStart(2, "0");
//     const cleanedDateStr = `${datePart}T${paddedHours}:${rest.split(" +")[0]}Z`;
  
//     // console.log("Cleaned Date String:", cleanedDateStr); // Log the cleaned date string
  
//     // Convert the cleaned string to a Date object
//     const parsedDate = new Date(cleanedDateStr);
//     // Parse the input date and time in UTC, then convert to IST
//     const istDate = utc(parsedDate).tz("Asia/Kolkata");
  
//     // Format the date to "YYYY-MM-DD hh:mm A"
//     return istDate.format("YYYY-MM-DD hh:mm A");
//   }
  
//   // Example usage
//   const dateStr = "2024-09-30 5:57:40.705 +00:00:00";
//   console.log(convertToIndianTime(dateStr)); // Expected output: "2024-09-30 11:27 AM"
  