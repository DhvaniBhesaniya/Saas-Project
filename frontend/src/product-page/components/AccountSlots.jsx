import * as React from "react";

import { useQuery } from "@tanstack/react-query";
import { Avatar } from "@mui/material";
export default function AccountSlots() {
  const { data: authUser } = useQuery({ queryKey: ["authUser"] });

  return <Avatar alt="Remy Sharp" src={authUser.profileImg || "/goku.jpg"} />;
}
