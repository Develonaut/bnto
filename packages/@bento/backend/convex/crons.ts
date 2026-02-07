import { cronJobs } from "convex/server";
import { internal } from "./_generated/api";

const crons = cronJobs();

// Reset run counters on the 1st of every month at 00:00 UTC.
crons.monthly(
  "reset run counters",
  { day: 1, hourUTC: 0, minuteUTC: 0 },
  internal.users.resetRunCounters,
);

export default crons;
