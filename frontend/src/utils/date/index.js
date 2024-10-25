import utc from "moment-timezone";

export const formatPostDate = (createdAt) => {
  const currentDate = new Date();
  const createdAtDate = new Date(createdAt);

  const timeDifferenceInSeconds = Math.floor(
    (currentDate - createdAtDate) / 1000
  );
  const timeDifferenceInMinutes = Math.floor(timeDifferenceInSeconds / 60);
  const timeDifferenceInHours = Math.floor(timeDifferenceInMinutes / 60);
  const timeDifferenceInDays = Math.floor(timeDifferenceInHours / 24);

  if (timeDifferenceInDays > 1) {
    return createdAtDate.toLocaleDateString("en-US", {
      month: "short",
      day: "numeric",
    });
  } else if (timeDifferenceInDays === 1) {
    return "1d";
  } else if (timeDifferenceInHours >= 1) {
    return `${timeDifferenceInHours}h`;
  } else if (timeDifferenceInMinutes >= 1) {
    return `${timeDifferenceInMinutes}m`;
  } else {
    return "Just now";
  }
};

// export const formatMemberSinceDate = (createdAt) => {
// 	const date = new Date(createdAt);
// 	const months = [
// 		"January",
// 		"February",
// 		"March",
// 		"April",
// 		"May",
// 		"June",
// 		"July",
// 		"August",
// 		"September",
// 		"October",
// 		"November",
// 		"December",
// 	];
// 	const month = months[date.getMonth()];
// 	const year = date.getFullYear();
// 	return `Joined ${month} ${year}`;
// };

// console.log(formatMemberSinceDate("2024-09-30 5:57:40.705 +00:00:00"));

export const formatMemberSinceDate = (createdAt) => {
  if (!createdAt) {
    // If createdAt is undefined, set a default date to January 1st 12:00 AM
    createdAt = "2024-01-01 00:00:00.000 +00:00:00";
  }

  // First, clean up the createdAt string to a more standard ISO format
  let [datePart, timePart] = createdAt.split(" ");
  const [hours, rest] = timePart.split(":");

  // Ensure the hours are zero-padded to two digits
  const paddedHours = hours.padStart(2, "0");
  const cleanedDateStr = `${datePart}T${paddedHours}:${rest.split(" +")[0]}Z`;

  // console.log("Cleaned Date String:", cleanedDateStr); // Log the cleaned date string

  // Convert the cleaned string to a Date object
  const parsedDate = new Date(cleanedDateStr);
  // console.log("Parsed Date:", parsedDate); // Log the parsed date
  // 2024-09-30T05:57:00.000Z
  // Then use the parsed date to extract day, month, and year
  const day = parsedDate.getDate();
  const months = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
  ];
  const month = months[parsedDate.getMonth()];
  const year = parsedDate.getFullYear();

  // Fallback if date parsing fails
  if (!month || isNaN(year)) {
    return "Invalid date";
  }

  // Function to determine the ordinal suffix (st, nd, rd, th)
  const getOrdinalSuffix = (day) => {
    if (day > 3 && day < 21) return "th"; // Covers 11th, 12th, 13th, etc.
    switch (day % 10) {
      case 1:
        return "st";
      case 2:
        return "nd";
      case 3:
        return "rd";
      default:
        return "th";
    }
  };

  // Combine the day with its ordinal suffix
  const dayWithSuffix = `${day}${getOrdinalSuffix(day)}`;

  return `Joined ${dayWithSuffix} ${month} ${year}`;
};

//   console.log(formatMemberSinceDate("2024-09-30 05:57:40.705 +00:00"));

export const convertToIndianTime = (createdAt) => {
  // First, clean up the createdAt string to a more standard ISO format
  let [datePart, timePart] = createdAt.split(" ");
  const [hours, rest] = timePart.split(":");

  // Ensure the hours are zero-padded to two digits
  const paddedHours = hours.padStart(2, "0");
  const cleanedDateStr = `${datePart}T${paddedHours}:${rest.split(" +")[0]}Z`;

  // console.log("Cleaned Date String:", cleanedDateStr); // Log the cleaned date string

  // Convert the cleaned string to a Date object
  const parsedDate = new Date(cleanedDateStr);
  // Parse the input date and time in UTC, then convert to IST
  const istDate = utc(parsedDate).tz("Asia/Kolkata");

  // Format the date to "YYYY-MM-DD hh:mm A"
  return istDate.format("YYYY-MM-DD hh:mm A");
}

// Example usage
// const dateStr = "2024-09-30 8:57:40.705 +00:00:00";
// console.log(convertToIndianTime(dateStr)); // Expected output: "2024-09-30 11:27 AM"
